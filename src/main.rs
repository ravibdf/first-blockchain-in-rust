mod balances;
mod system;

fn main() {
    let mut pallet = balances::Pallet::new();
    pallet.set_balance("daniel".to_string(), 2);

    let balance = pallet.balance("daniel".to_string());
    println!("balance: {}", balance);
}
