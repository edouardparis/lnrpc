include!(concat!(env!("OUT_DIR"), "/lnrpc.rs"));

pub mod interceptor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lnrpc_imported() {
        assert_eq!(WalletBalanceRequest {}, WalletBalanceRequest {});
    }
}
