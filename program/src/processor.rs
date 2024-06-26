use borsh::{BorshDeserialize, BorshSerialize};
use crate::error::Error::UnknownOperation;
use crate::instruction::Instruction;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Account {
    result: u32,
}

#[derive(PartialEq)]
enum CalculationType {
    Add,
    Subtract,
}

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = Instruction::try_from_slice(instruction_data)?;

        match instruction {
            Instruction::Add { number_a, number_b } => Self::process_calculate(
                accounts,
                number_a,
                number_b,
                program_id,
                CalculationType::Add,
            ),
            Instruction::Subtract { number_a, number_b } => Self::process_calculate(
                accounts,
                number_a,
                number_b,
                program_id,
                CalculationType::Subtract,
            ),
        }
    }

    fn process_calculate(
        accounts: &[AccountInfo],
        number_a: u32,
        number_b: u32,
        program_id: &Pubkey,
        calculation_type: CalculationType,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        let account = next_account_info(accounts_iter)?;

        if account.owner != program_id {
            msg!("Account is not owned by the program");
            return Err(ProgramError::IncorrectProgramId);
        }

        msg!("Numbers are {} and {}", number_a, number_b);

        let mut calculator_account = Account::try_from_slice(&account.data.borrow())?;

        if calculation_type == CalculationType::Add {
            let sum = number_a + number_b;
            calculator_account.result = sum;
        } else if calculation_type == CalculationType::Subtract {
            let difference = number_a - number_b;
            calculator_account.result = difference;
        } else {
            msg!("Unrecognized operation");
            return Err(UnknownOperation.into());
        }

        // Serialize the updated account data back into the account
        calculator_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

        msg!("Result added");

        Ok(())
    }
}
