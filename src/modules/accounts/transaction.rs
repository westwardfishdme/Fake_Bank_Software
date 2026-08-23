pub enum TransferFailures {
    FailedToSend,
    AccountDoesNotExist,
    InsufficientFunds,
    NoTransactionHistory,
}
pub trait Transfers {
    fn send<T: ToString>(&self, recipient: T) -> Result<(), TransferFailures>;
    fn request<T: ToString>(&self, recipient: T) -> Result<(), TransferFailures>;
    fn reverse(&self) -> Result<(), TransferFailures>;
}
