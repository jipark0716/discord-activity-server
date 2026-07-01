fn main() -> anyhow::Result<()> {
    let protoc = protoc_bin_vendored::protoc_bin_path()?;

    let mut config = prost_build::Config::new();
    config.protoc_executable(protoc);

    tonic_prost_build::configure().compile_with_config(
        config,
        &["proto/gateway.proto", "proto/auth.proto", "proto/message.proto"],
        &["proto"],
    )?;

    println!("cargo:rerun-if-changed=proto/gateway.proto");

    Ok(())
}