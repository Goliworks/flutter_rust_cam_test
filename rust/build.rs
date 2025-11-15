use burn_import::onnx::ModelGen;

fn main() {
    let file = std::env::var("ONNX_MODEL_PATH").unwrap();
    ModelGen::new()
        .input(&file)
        .out_dir("model/")
        .embed_states(true)
        .half_precision(false)
        .record_type(burn_import::onnx::RecordType::Bincode) // ← Utiliser Bincode au lieu de NamedMpk
        .run_from_script();
}
