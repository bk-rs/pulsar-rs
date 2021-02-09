extern crate protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src/protos/protobuf")
        .inputs(&["protos/PulsarApi.proto"])
        .include("protos")
        .run()
        .unwrap();
}
