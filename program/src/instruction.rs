use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum BlogInstruction {
    /// Initialize a new blog
    /// Accounts:
    /// 0. `[signer, writable]` Blog authority
    /// 1. `[writable]` Blog account to initialize
    /// 2. `[]` System program
    InitializeBlog { title: String, description: String },

    /// Create a new blog post
    /// Accounts:
    /// 0. `[signer]` Post author
    /// 1. `[writable]` Post account to initialize
    /// 2. `[writable]` Blog account
    /// 3. `[]` System program
    CreatePost {
        title: String,
        content: String,
        arweave_hash: String,
    },

    /// Update an existing blog post
    /// Accounts:
    /// 0. `[signer]` Post author
    /// 1. `[writable]` Post account
    UpdatePost {
        title: Option<String>,
        content: Option<String>,
        arweave_hash: Option<String>,
    },
}

impl BlogInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        Self::try_from_slice(input).map_err(|_| ProgramError::InvalidInstructionData)
    }
} 