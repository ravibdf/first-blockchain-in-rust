mod balances;
mod system;
mod support;

mod types {
    use crate::support;

    pub type Balance = u128;
    pub type AccountId = String;
    pub type BlockNumber = u64;
    pub type Nonce = u32;

    pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrinsic>;

    
}

pub enum RuntimeCall{}
#[derive(Debug)]
pub struct Runtime {
    balances: balances::Pallet<Self>,
    system: system::Pallet<Self> ,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            balances: balances::Pallet::new(),
            system: system::Pallet::new(),
        }
    }
}

impl  system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    
}

impl balances::Config for Runtime {
    type AccountId = types::AccountId;
    type Balance = types::Balance;
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
