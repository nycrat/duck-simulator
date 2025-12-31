use protobuf_codegen::Codegen;

fn main() {
    Codegen::new()
        .pure()
        .include("../protos")
        .input("../protos/duck.proto")
        .input("../protos/update.proto")
        .cargo_out_dir("protos")
        .run_from_script();
}
