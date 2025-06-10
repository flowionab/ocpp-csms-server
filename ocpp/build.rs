fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile_protos(
            &[
                "../proto/ocpp_service.proto",
                "../proto/client/csms_server_client_service.proto",
            ],
            &["../proto"],
        )
        .unwrap();
    Ok(())
}
