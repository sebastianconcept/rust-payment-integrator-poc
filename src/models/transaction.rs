use serde::Deserialize;

// type, client, tx, amount
#[derive(Debug, Deserialize)]
pub struct Transaction {
    kind: String,
    client_id: u16,
    id: u32,
    amount: f32,
}
