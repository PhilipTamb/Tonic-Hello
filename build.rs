// This tells tonic-build to compile your protobufs when you build your Rust project. 
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos(
        "proto/hello.proto")?;
    Ok(())
}
