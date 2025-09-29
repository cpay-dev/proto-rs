fn main() {
  tonic_prost_build::configure()
    .build_client(true)
    .out_dir(concat!(env!("WORKSPACE_ROOT"), "/crates/proto/src"))
    .include_file(&"lib.rs")
    .compile_protos(
      &[
        // api/v1
        "api/v1/kms/service.proto",

        "api/v1/merchant/asset.proto",
        "api/v1/merchant/chain.proto",
        "api/v1/merchant/payment-intent.proto",
        "api/v1/merchant/asset-service.proto",
        "api/v1/merchant/chain-service.proto",
        "api/v1/merchant/payment-intent-service.proto",
        
        "api/v1/wallet/service.proto",

        // blockchain/v1
        "blockchain/v1/indexer/block.proto",
        "blockchain/v1/indexer/parsed-block.proto",
        
        "blockchain/v1/block.proto",
        "blockchain/v1/chain.proto",
      ],
      &[&std::env::var("PROTO_SRC_DIR").expect("PROTO_SRC_DIR environment variable must be set")],
    )
    .expect("must compile proto models");
}
