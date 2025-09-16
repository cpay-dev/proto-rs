fn main() {
  tonic_prost_build::configure()
    .build_client(true)
    .out_dir(concat!(env!("WORKSPACE_ROOT"), "/crates/proto/src"))
    .include_file(&"lib.rs")
    .compile_protos(
      &[
        // api/v1
        "api/v1/blockchain/asset.proto",
        "api/v1/blockchain/chain.proto",
        "api/v1/blockchain/chain_id.proto",
        "api/v1/merchant/service.proto",
      ],
      &[&std::env::var("PROTO_SRC_DIR").expect("PROTO_SRC_DIR environment variable must be set")],
    )
    .expect("must compile proto models");
}
