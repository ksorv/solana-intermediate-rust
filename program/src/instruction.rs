use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub enum Instruction {
    Calculator { number_a: u32, number_b: u32 },
}
