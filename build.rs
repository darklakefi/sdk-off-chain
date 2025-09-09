/* fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = &["src/proto/integrations.proto"];
    let includes = &["src/proto"];

    // Create a prost_build::Config object
    let mut config = prost_build::Config::new();

    // Use prost_validate_build's builder to configure this prost_build object.
    prost_validate_build::Builder::new().configure(&mut config, files, includes)?;

    // Set the service generator from tonic_build
    // config.service_generator(tonic_build::configure().service_generator());
    tonic_build::configure().compile_protos(files, includes)?;

    // Compile the protos with the now-configured 'config' object
    config.compile_protos(files, includes)?;

    Ok(())
}
 */

use std::io::Result;

fn main() -> Result<()> {
    let protos = &["src/proto/darklake/integrations/v1/client.proto"];
    let includes = &["src/proto"];

    tonic_prost_build::configure()
        .build_server(false)
        .compile_protos(protos, includes)?;
    Ok(())
}
