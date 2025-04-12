use std::collections::BTreeMap;
use num::{CheckedAdd, One, Zero};

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + Copy + CheckedAdd + One;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::BlockNumber>,    
}

impl <T: Config> Pallet <T> 
where 
    // BlockNumber: Zero + Copy + CheckedAdd + One,
    // AccountId: Ord + Clone, 
{
    pub fn new() -> Self {
        Pallet {
            block_number: T::BlockNumber::zero(),  
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn increment_block_number(&mut self) {
        self.block_number = self.block_number.checked_add(&T::BlockNumber::one()).expect("overflow");
    }

    pub fn increment_nonce(&mut self, account: &T::AccountId) {        
        *self.nonce.entry(account.clone())
        .or_insert(T::BlockNumber::zero()) = 
        self.nonce.get(account).unwrap_or(&T::BlockNumber::zero())
                .checked_add(&T::BlockNumber::one())
                .expect("overflow");        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct TestCongig;

    impl Config for TestCongig {
        type AccountId = String;
        type BlockNumber = u32;
    }


    // Define concrete types for testing
    type TestBlockNumber = u32;
    type TestAccountId = String;

    #[test]
    fn test_initial_state() {
        let system: Pallet<TestCongig> = Pallet::new();
        assert_eq!(system.block_number(),0);
        assert!(system.nonce.is_empty());
    }

    #[test]
    fn test_block_number_increment() {
        let mut system: Pallet<TestCongig> = Pallet::new();
        system.increment_block_number();
        assert_eq!(system.block_number(), 1);
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn test_block_number_overflow() {
        let mut system: Pallet<TestCongig> = Pallet::new();
        system.block_number = TestBlockNumber::MAX;
        system.increment_block_number();
    }

    #[test]
    fn test_nonce_operations() {
        let mut system: Pallet<TestCongig> = Pallet::new();
        let account = "daniel";

        // First increment create entry
        system.increment_nonce(&account.to_string());
        assert_eq!(system.nonce.get(account), Some(&1));

        // Second increment update entry
        system.increment_nonce(&account.to_string());
        assert_eq!(system.nonce.get(account), Some(&2));

        // Different account
        assert_eq!(system.nonce.get("alice"), None);
    }
}