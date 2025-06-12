use rand::Rng;

pub fn create_transaction_id() -> i32 {
    let mut rng = rand::rng();
    rng.random::<i32>()
}
