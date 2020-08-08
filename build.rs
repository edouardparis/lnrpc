use tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile(
            &["lnd/lnrpc/rpc.proto"],
            &["lnd/", "grpc-gateway/third_party/googleapis/"],
        )?;
    Ok(())
}
