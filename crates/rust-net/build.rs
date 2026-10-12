use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::compile_protos("proto/rendezvous.proto")?;
    
    let include_dir = PathBuf::from("./include");
    fs::create_dir_all(&include_dir).unwrap();
    let output_path = include_dir.join("rust_net.h");

    let config = cbindgen::Config::from_file("./cbindgen.toml")
        .expect("Failed to load cbindgen.toml");

    cbindgen::Builder::new()
        .with_crate(".")
        .with_config(config)
        .generate()
        .expect("Unable to generate C header")
        .write_to_file(output_path);

    Ok(())
}
