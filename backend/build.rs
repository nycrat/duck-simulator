use protobuf_codegen::Codegen;

fn main() {
    Codegen::new()
        .pure()
        .include("src/protos")
        .input("src/protos/protos.proto")
        .cargo_out_dir("protos")
        .run_from_script();
}
