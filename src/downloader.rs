use std::process::Command;
use std::fs::File;
use std::io::copy;
use reqwest::blocking::Client;
use url::Url;

pub fn download_yt (url: &str) -> Result<(), String> {
    let status = Command::new("yt-dlp")
            .arg(url)
            .status()
            .map_err(|e| format!("Youtube not found or failed: {}", e))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("Youtube failed with status: {}", status))
    }
}

pub fn download_file (url: &str) -> Result<String, String> {
    let parsed = Url::parse(url).map_err(|e| format!("Invalid URL: {}", e))?;
    let filename = parsed
        .path_segments()
        .and_then(|segments| segments.last())
        .filter(|name| !name.is_empty())
        .unwrap_or("downloaded_file");

    let client = Client::builder()
        .user_agent("rget/0.1")
        .build()
        .map_err(|e| format!("Failed to build client: {}", e))?;

    let mut res = client.get(url).send().map_err(|e| format!("Request error: {}", e))?;
    if !res.status().is_success() {
        return Err(format!("HTTP error: {}", res.status()));
    }

    let mut file = File::create(filename).map_err(|e| format!("File create error: {}", e))?;
    copy(&mut res, &mut file).map_err(|e| format!("Format error: {}", e))?;

    Ok(filename.to_string())
}