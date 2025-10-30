fn main() {
	tonic_prost_build::configure()
		.build_client(true)
		.out_dir(concat!(env!("WORKSPACE_ROOT"), "/crates/proto/src"))
		.include_file(&"lib.rs")
		.compile_protos(
			&[
				// api/v1/authn
				"api/v1/authn/init_auth.proto",
				"api/v1/authn/provider.proto",
				"api/v1/authn/service.proto",
			],
			&[&std::env::var("PROTO_SRC_DIR").expect("PROTO_SRC_DIR environment variable must be set")],
		)
		.expect("must compile proto models");
}
