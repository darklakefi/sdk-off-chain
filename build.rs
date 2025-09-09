// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use std::io::Result;

fn main() -> Result<()> {
    let protos = &["src/proto/darklake/v1/api.proto"];
    let includes = &["src/proto"];

    tonic_prost_build::configure()
        .build_server(false)
        .compile_protos(protos, includes)?;
    Ok(())
}
