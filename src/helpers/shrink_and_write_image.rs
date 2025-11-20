use std::process::Command;

pub fn shrink_and_write_image(
    input_path: String,
    quality: String,
    size: String,
    base_dir: String,
    url: String
) -> Result<String, ()> {
    let result = Command::new("sharp")
        .args([
            "--input", input_path.as_str(),
            "--format", "avif",
            "--quality", &quality,
            "resize", &size,
            "--output", &format!("{}{}", base_dir, url)
        ])
        .stdout(std::process::Stdio::null())
        .status();

    match result {
        Ok(status) if status.success() => Ok(url),
        _ => Err(())
    }
}
