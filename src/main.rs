use serpapi_search_rust::serp_api_search::SerpApiSearch;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::error::Error;
use std::env;

mod my_secret;

async fn search_query(query: &str) -> Result<(), Box<dyn Error>> {
    let mut params = HashMap::<String, String>::new();
    params.insert("engine".to_string(), "google".to_string());
    params.insert("q".to_string(), query.to_string());
    params.insert("google_domain".to_string(), "google.com".to_string());
    params.insert("gl".to_string(), "us".to_string());
    params.insert("hl".to_string(), "en".to_string());

    let search = SerpApiSearch::google(params, my_secret::SERP_API_KEY.to_string());
    
    let results = search.json().await?;
    println!("results received");
    println!("--- JSON ---");
    println!(" - results: {}", results);

    // Convert results to a JSON string
    let json_string = serde_json::to_string(&results)?;

    // Define the file path
    let file_path = "data/results.json";

    // Write the JSON string to the file
    let mut file = File::create(file_path)?;
    file.write_all(json_string.as_bytes())?;

    println!("Results have been written to {}", file_path);

    Ok(())
    }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <query>", args[0]);
        std::process::exit(1);
    }

    let query = &args[1];
    search_query(query).await
}