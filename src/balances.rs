use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero};

pub trait Config: crate::system::Config{
    type Balance: CheckedAdd + CheckedSub + Zero + Copy;
}
#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl <T: Config> Pallet<T> 
{
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, account: T::AccountId, balance: T::Balance) {
        self.balances.insert(account, balance);
    }

    pub fn balance(&self, account: T::AccountId) -> T::Balance {
        self.balances.get(&account).copied().unwrap_or(T::Balance::zero())
    }

    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = *self.balances.get(&caller)
                                                        .unwrap_or(&T::Balance::zero());
        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("insufficient balance")?;

        let to_balance = *self.balances.get(&to).unwrap_or(&T::Balance::zero());
        let new_to_balance = to_balance
            .checked_add(&amount)
            .ok_or("overflow")?;

        self.balances.insert(caller, new_caller_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestConfig;

    impl Config for TestConfig {
        type AccountId = String;
        type Balance = u32;
    }

    #[test]
    fn initial_balance() {
        let mut pallet: Pallet<TestConfig> = Pallet::new();
        pallet.set_balance("daniel".to_string(), 2);

        assert_eq!(pallet.balance("daniel".to_string()), 2);
    }

    #[test]
    fn transfer_balance() {
        let mut balances: Pallet<TestConfig> = Pallet::new();

        assert_eq!(
            balances.transfer("daniel".to_string(), "vini".to_string(),10),
            Err("insufficient balance")
        );

        balances.set_balance("daniel".to_string(), 10);
        assert_eq!(balances.transfer("daniel".to_string(), "vini".to_string(), 5), Ok(()));

        balances.set_balance("vini".to_string(), u32::MAX);
        assert_eq!(balances.transfer("daniel".to_string(), "vini".to_string(), 5),
        Err("overflow"));
    }
}