use lnrpc::lightning_client::LightningClient;
use lnrpc::WalletBalanceRequest;
use tokio_rustls::rustls::ClientConfig;
use tonic::transport::{Channel, ClientTlsConfig};

// cargo run --example get_wallet_balance -- http://localhost:10001 \
// /home/user/.polar/networks/2/volumes/lnd/alice/tls.cert \
// home/user/.polar/networks/2/volumes/lnd/alice/data/chain/bitcoin/regtest/admin.macaroon

const ALPN_H2: &str = "h2";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        println!("{:?}", args);
        println!("args: <address> <cert> <macaroon>");
        return Ok(());
    }
    let cert = tokio::fs::read(&args[2]).await?;
    let macaroon = tokio::fs::read(&args[3]).await?;

    let mut config = ClientConfig::new();
    config.set_protocols(&[Vec::from(&ALPN_H2[..])]);
    let mut buf = std::io::Cursor::new(&cert[..]);
    config.root_store.add_pem_file(&mut buf).unwrap();
    let mut danger = config.dangerous();
    danger.set_certificate_verifier(std::sync::Arc::new(mock::MockVerifier {}));
    let tls = ClientTlsConfig::new()
        .domain_name("localhost")
        .rustls_client_config(danger.cfg.clone());

    let channel = Channel::from_shared(args[1].clone())?
        .tls_config(tls)?
        .connect()
        .await?;

    let mac = lnrpc::interceptor::macaroon(&macaroon);
    let mut client = LightningClient::with_interceptor(channel, mac);
    let balance = client.wallet_balance(WalletBalanceRequest {}).await?;
    println!("{:?}", balance);
    Ok(())
}

pub mod mock {
    use tokio_rustls::rustls::{
        Certificate, RootCertStore, ServerCertVerified, ServerCertVerifier, TLSError,
    };
    use webpki::DNSNameRef;
    pub struct MockVerifier;

    /// MockVerifier is a rustls::ServerCertVerified which does not verify the certificate.
    /// Sadly a self signed certificate is rejected with WebPKIError (CAUsedAsEndEntity),
    /// see https://github.com/briansmith/webpki/issues/114 .
    impl ServerCertVerifier for MockVerifier {
        fn verify_server_cert(
            &self,
            _roots: &RootCertStore,
            _presented_certs: &[Certificate],
            _dns_name: DNSNameRef,
            _ocsp_response: &[u8],
        ) -> Result<ServerCertVerified, TLSError> {
            return Ok(ServerCertVerified::assertion());
        }
    }
}
