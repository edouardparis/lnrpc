use hex;
use tonic::{Request, Status};

/// macaroon returns an interceptor which inserts
/// the lnd macaroon in the request metadata.
pub fn macaroon(mac: &[u8]) -> tonic::Interceptor {
    let hex_mac = hex::encode(mac);
    let f = move |r: Request<()>| -> Result<Request<()>, Status> {
        let mut req = r;
        let metadata = req.metadata_mut();
        metadata.insert("macaroon", hex_mac.parse().unwrap());
        Ok(req)
    };
    return tonic::Interceptor::new(f);
}
