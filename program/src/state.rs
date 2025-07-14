
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]

pub struct VaultState {
    pub initialized: bool,
    pub counter: u64,
}