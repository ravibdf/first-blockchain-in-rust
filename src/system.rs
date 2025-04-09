use std::collections::BTreeMap;

pub struct Pallet {
    block_number: u64,
    nonce: BTreeMap<String, u64>,    
}

impl Pallet {
    pub fn new() -> Self {
        Pallet {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> u64 {
        self.block_number
    }

    pub fn increment_block_number(&mut self) {
        self.block_number = self.block_number.checked_add(1).expect("overflow");
    }

    pub fn increment_nonce(&mut self, account: &String) {        
        *self.nonce.entry(account.clone()).or_insert(0) += 1;        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let system = Pallet::new();
        assert_eq!(system.block_number(),0);
        assert!(system.nonce.is_empty());
    }

    #[test]
    fn test_block_number_increment() {
        let mut system = Pallet::new();
        system.increment_block_number();
        assert_eq!(system.block_number(), 1);
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn test_block_number_overflow() {
        let mut system = Pallet::new();
        system.block_number = u64::MAX;
        system.increment_block_number();
    }

    #[test]
    fn test_nonce_operations() {
        let mut system = Pallet::new();
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