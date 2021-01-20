fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().build_server(false).compile(
        &[
            "proto/google/pubsub/v1/pubsub.proto",
            "proto/google/pubsub/v1beta2/pubsub.proto",
        ],
        &["proto"],
    )?;
    Ok(())
}
