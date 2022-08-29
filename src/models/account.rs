use super::transaction::{ClientID, Amount};

pub struct Account {
    client_id: ClientID,
    available: Amount,
    held: Amount,
    locked: bool,
}
