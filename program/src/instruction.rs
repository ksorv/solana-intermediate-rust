use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub enum Instruction {
    Add { number_a: u32, number_b: u32 },
    Subtract { number_a: u32, number_b: u32 },
}
