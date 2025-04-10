fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile_protos(&["../proto/api_service.proto"], &["../proto"])
        .unwrap();
    Ok(())
}
