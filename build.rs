fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../kenja-proto/anime_search.proto")?;
    Ok(())
}
