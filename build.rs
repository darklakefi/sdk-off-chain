use std::io::Result;

fn main() -> Result<()> {
    let protos = &["src/proto/darklake/v1/api.proto"];
    let includes = &["src/proto"];

    tonic_prost_build::configure()
        .build_server(false)
        .compile_protos(protos, includes)?;
    Ok(())
}
