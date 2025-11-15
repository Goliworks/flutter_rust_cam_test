use burn_import::onnx::ModelGen;

fn main() {
    let file = std::env::var("ONNX_MODEL_PATH").unwrap();
    ModelGen::new()
        .input(&file)
        .out_dir("model/")
        .run_from_script();
}
