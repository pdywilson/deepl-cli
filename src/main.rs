use std::fs;
use std::io::Read;
use reqwest;
use reqwest::header;
use clap::Parser;
use serde_json::Value;

#[derive(Parser)]
struct Cli {
    word: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let client = reqwest::Client::new();

    let file_path = "/Users/pdywilson/dev/fun/deepl-cli/key";
    let mut file = fs::File::open(file_path).expect("Failed to open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read file");
    let key = format!("DeepL-Auth-Key {}", content);

    let params = [
        ("text", args.word),
        ("target_lang", "PT-BR".to_owned()),
        ("source_lang", "EN".to_owned()),
    ];
    let response = client
        .post("https://api-free.deepl.com/v2/translate")
        .form(&params)
        .header(header::AUTHORIZATION, key.trim())
        .send()
        .await?;

    if response.status().is_success() {
        // let body = response.text().await?;
        let json: Value = response.json().await?;
        let body = json["translations"][0]["text"].as_str().unwrap_or("");
        println!("It's: {}, cuzao!", body);
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}
