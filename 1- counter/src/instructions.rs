use borsh::BorshDeserialize as BorshDeserializeTrait;
use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct InputArgs {
    pub value: u32,
}

pub enum CounterInstructions {
    Increment(InputArgs),
    Decrement(InputArgs),
    Update(InputArgs),
    Reset,
}

impl CounterInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => Self::Increment(InputArgs::try_from_slice(rest).unwrap()),
            1 => Self::Decrement(InputArgs::try_from_slice(rest).unwrap()),
            2 => Self::Update(InputArgs::try_from_slice(rest).unwrap()),
            3 => Self::Reset,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
