use crate::{instruction::BlogInstruction, state::{Blog, BlogPost}};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
    clock::Clock,
    msg,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = BlogInstruction::unpack(instruction_data)?;

        match instruction {
            BlogInstruction::InitializeBlog { title, description } => {
                Self::process_initialize_blog(program_id, accounts, title, description)
            }
            BlogInstruction::CreatePost { title, content, arweave_hash } => {
                Self::process_create_post(program_id, accounts, title, content, arweave_hash)
            }
            BlogInstruction::UpdatePost { title, content, arweave_hash } => {
                Self::process_update_post(program_id, accounts, title, content, arweave_hash)
            }
        }
    }

    fn process_initialize_blog(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        title: String,
        description: String,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let authority_info = next_account_info(account_info_iter)?;
        let blog_info = next_account_info(account_info_iter)?;
        let system_program_info = next_account_info(account_info_iter)?;

        if !authority_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if title.len() > Blog::MAX_TITLE_LENGTH {
            return Err(ProgramError::InvalidInstructionData);
        }

        if description.len() > Blog::MAX_DESCRIPTION_LENGTH {
            return Err(ProgramError::InvalidInstructionData);
        }

        let rent = Rent::get()?;
        let space = Blog::get_size(&title, &description);
        let lamports = rent.minimum_balance(space);

        invoke(
            &system_instruction::create_account(
                authority_info.key,
                blog_info.key,
                lamports,
                space as u64,
                program_id,
            ),
            &[
                authority_info.clone(),
                blog_info.clone(),
                system_program_info.clone(),
            ],
        )?;

        let clock = Clock::get()?;
        let blog = Blog {
            authority: *authority_info.key,
            title,
            description,
            post_count: 0,
            created_at: clock.unix_timestamp,
        };

        blog.serialize(&mut &mut blog_info.data.borrow_mut()[..])?;

        msg!("Blog initialized successfully");
        Ok(())
    }

    fn process_create_post(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        title: String,
        content: String,
        arweave_hash: String,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let author_info = next_account_info(account_info_iter)?;
        let post_info = next_account_info(account_info_iter)?;
        let blog_info = next_account_info(account_info_iter)?;
        let system_program_info = next_account_info(account_info_iter)?;

        if !author_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if title.len() > BlogPost::MAX_TITLE_LENGTH {
            return Err(ProgramError::InvalidInstructionData);
        }

        if content.len() > BlogPost::MAX_CONTENT_LENGTH {
            return Err(ProgramError::InvalidInstructionData);
        }

        if arweave_hash.len() > BlogPost::MAX_ARWEAVE_HASH_LENGTH {
            return Err(ProgramError::InvalidInstructionData);
        }

        // Verify blog ownership
        let mut blog_data = blog_info.try_borrow_mut_data()?;
        let mut blog = Blog::try_from_slice(&blog_data)?;

        let rent = Rent::get()?;
        let space = BlogPost::get_size(&title, &content, &arweave_hash);
        let lamports = rent.minimum_balance(space);

        invoke(
            &system_instruction::create_account(
                author_info.key,
                post_info.key,
                lamports,
                space as u64,
                program_id,
            ),
            &[
                author_info.clone(),
                post_info.clone(),
                system_program_info.clone(),
            ],
        )?;

        let clock = Clock::get()?;
        let blog_post = BlogPost {
            author: *author_info.key,
            blog: *blog_info.key,
            title,
            content,
            arweave_hash,
            created_at: clock.unix_timestamp,
            updated_at: clock.unix_timestamp,
        };

        blog_post.serialize(&mut &mut post_info.data.borrow_mut()[..])?;

        // Update blog post count
        blog.post_count += 1;
        blog.serialize(&mut &mut blog_data[..])?;

        msg!("Blog post created successfully");
        Ok(())
    }

    fn process_update_post(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        title: Option<String>,
        content: Option<String>,
        arweave_hash: Option<String>,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let author_info = next_account_info(account_info_iter)?;
        let post_info = next_account_info(account_info_iter)?;

        if !author_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut post_data = post_info.try_borrow_mut_data()?;
        let mut blog_post = BlogPost::try_from_slice(&post_data)?;

        if blog_post.author != *author_info.key {
            return Err(ProgramError::InvalidAccountData);
        }

        if let Some(new_title) = title {
            if new_title.len() > BlogPost::MAX_TITLE_LENGTH {
                return Err(ProgramError::InvalidInstructionData);
            }
            blog_post.title = new_title;
        }

        if let Some(new_content) = content {
            if new_content.len() > BlogPost::MAX_CONTENT_LENGTH {
                return Err(ProgramError::InvalidInstructionData);
            }
            blog_post.content = new_content;
        }

        if let Some(new_arweave_hash) = arweave_hash {
            if new_arweave_hash.len() > BlogPost::MAX_ARWEAVE_HASH_LENGTH {
                return Err(ProgramError::InvalidInstructionData);
            }
            blog_post.arweave_hash = new_arweave_hash;
        }

        let clock = Clock::get()?;
        blog_post.updated_at = clock.unix_timestamp;

        blog_post.serialize(&mut &mut post_data[..])?;

        msg!("Blog post updated successfully");
        Ok(())
    }
} 