use std::env;
use std::path::PathBuf;

fn main() {
    let descriptor_path =
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("urlfrontier_descriptor.bin");

    tonic_build::configure()
        .file_descriptor_set_path(&descriptor_path)
        .compile(&["urlfrontier.proto"], &["."])
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));

    // serde implementations following the protobuf JSON mapping, so that the JSON
    // read and written is the same as with the Java client
    let descriptor_set = std::fs::read(&descriptor_path).unwrap();
    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set)
        .unwrap()
        .build(&[".urlfrontier"])
        .unwrap_or_else(|e| panic!("Failed to generate the serde code {:?}", e));
}
