use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    prost_build::compile_protos(&["proto/messages.proto"], &["proto/"])?;

    Ok(())
}
