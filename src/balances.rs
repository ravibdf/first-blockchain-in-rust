use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, account: String, balance: u128) {
        self.balances.insert(account, balance);
    }

    pub fn balance(&self, account: String) -> u128 {
        self.balances.get(&account).copied().unwrap_or(0)
    }

    pub fn transfer(
        &mut self,
        caller: String,
        to: String,
        amount: u128,
    ) -> Result<(), String> {
        // let caller_balance = self.balance(caller.clone());
        // let caller_balance = self.balance(&caller);
        let caller_balance = *self.balances.get(&caller).unwrap_or(&0);
        let new_caller_balance = caller_balance
            .checked_sub(amount)
            .ok_or("insufficient balance")?;

        let to_balance = *self.balances.get(&to).unwrap_or(&0);
        let new_to_balance = to_balance
            .checked_add(amount)
            .ok_or("overflow")?;

        self.balances.insert(caller.to_string(), new_caller_balance);
        self.balances.insert(to.to_string(), new_to_balance);

        Ok(())

        // let caller_balance = *self.balances.get(&caller).unwrap_or(&0);
        // let to_balance = self.balance(to);

        // let new_caller_balance = caller_balance.checked_sub(amount).ok_or("insufficient balance")?;
        // let new_to_balance = to_balance.checked_add(amount).ok_or("overflow")?;

        // self.balances.insert(caller, new_caller_balance);
        // self.balances.insert(to, new_to_balance);

        // Ok(())
    }
}

#[test]
fn initial_balance() {
    let mut pallet = Pallet::new();
    pallet.set_balance("daniel".to_string(), 2);

    assert_eq!(pallet.balance("daniel".to_string()), 2);
}

#[test]
fn transfer_balance() {
    let mut balances = Pallet::new();

    assert_eq!(
        balances.transfer("daniel".to_string(), "vini".to_string(),10),
        Err("insufficient balance".to_string())
    );

    balances.set_balance("daniel".to_string(), 10);
    assert_eq!(balances.transfer("daniel".to_string(), "vini".to_string(), 5), Ok(()));

    balances.set_balance("vini".to_string(), u128::MAX);
    assert_eq!(balances.transfer("daniel".to_string(), "vini".to_string(), 5),
    Err("overflow".to_string()));
}