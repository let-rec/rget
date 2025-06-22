use downloader::{ download_file, download_yt };
use utils::{ yt_url };
use std::io::{ self, Write };

mod downloader;
mod utils;

fn main() {
    println!("Welcome to smart-dl CLI!");
    println!("Type 'download <url>' to download files or videos.");
    println!("Type 'exit' to quit.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input.");
            continue;
        }

        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        if let Some(url) = input.strip_prefix("download ") {
            let url = url.trim();
            if yt_url(url) {
                match download_yt(url) {
                    Ok(_) => println!("Youtube video download."),
                    Err(e) => eprintln!("Error: {}", e),
                }
            } else {
                match download_file(url) {
                    Ok(f) => println!("File downloader: {}", f),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        } else {
            println!("Unknown command. Use 'download <url>' or 'exit'");
        }
    }
}
