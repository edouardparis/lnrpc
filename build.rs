use prost_build::compile_protos;

fn main() {
    compile_protos(
        &["lnd/lnrpc/rpc.proto"],
        &["lnd/", "grpc-gateway/third_party/googleapis/"],
    )
    .unwrap();
}
