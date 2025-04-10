mod balances;
mod system;

type Balance = u128;
type AccountId = String;
#[derive(Debug)]
pub struct Runtime {
    balances: balances::Pallet<AccountId, Balance>,
    system: system::Pallet,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            balances: balances::Pallet::new(),
            system: system::Pallet::new(),
        }
    }
}

fn main() {    
    let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(alice.clone(), 100);

    runtime.system.increment_block_number();
    assert!(runtime.system.block_number() == 1);

    runtime.system.increment_nonce(&alice);
    if let Err(e) = runtime.balances.transfer(alice.clone(), bob.clone(), 30) {
        eprintln!("Transfer failed: {}", e);
    }

    runtime.system.increment_nonce(&alice);
    if let Err(e) = runtime.balances.transfer(alice.clone(), charlie.clone(), 20) {
        eprintln!("Transfer failed: {}", e);
    }

    println!("Alice's balances: {}", runtime.balances.balance(alice.clone()));
    println!("{:#?}", runtime);

}
