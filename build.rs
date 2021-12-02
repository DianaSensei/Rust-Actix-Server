fn main() -> Result<(), Box<dyn std::error::Error>> {
    rosetta_build::config()
        .source("vn", "locales/vn.json")
        .source("en", "locales/en.json")
        .fallback("vn")
        .output("target/rosetta_output.rs")
        .generate()?;

    Ok(())
}