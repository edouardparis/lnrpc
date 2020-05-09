include!(concat!(env!("OUT_DIR"), "/lnrpc.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lnrpc_imported() {
        assert_eq!(WalletBalanceRequest {}, WalletBalanceRequest {});
    }
}
