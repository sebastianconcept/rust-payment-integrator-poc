

type Result<T> = std::result::Result<T, InsufficientFoundsForWithdrawal>;

#[derive(Debug, Clone)]
pub struct InsufficientFoundsForWithdrawal;

#[derive(Debug, Clone)]
pub struct Withdrawal {}