use serpapi_search_rust::serp_api_search::SerpApiSearch;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::error::Error;
use std::env;

mod my_secret;

// Define structs to match the expected JSON structure
#[derive(Deserialize, Debug)]
struct OrganicResult {
    title: String,
    date: Option<String>,
    source: Option<String>,
    snippet: String,
    link: String,
}

#[derive(Deserialize, Debug)]
struct RelatedQuestion {
    title: String,
    date: Option<String>,
    question: String,
}

#[derive(Serialize, Debug)]
struct ParsedResult {
    title: String,
    date: Option<String>,
    source: Option<String>,
    content: String,
    r#type: String,
}

#[derive(Serialize, Debug)]
struct ParseOutput {
    content: Vec<ParsedResult>,
    links: Vec<String>,
}

async fn search_query(query: &str) -> Result<Value, Box<dyn Error>> {
    let mut params = HashMap::<String, String>::new();
    params.insert("engine".to_string(), "google".to_string());
    params.insert("q".to_string(), query.to_string());
    params.insert("google_domain".to_string(), "google.com".to_string());
    params.insert("gl".to_string(), "us".to_string());
    params.insert("hl".to_string(), "en".to_string());

    let search = SerpApiSearch::google(params, my_secret::SERP_API_KEY.to_string());
    
    let results = search.json().await?;

    Ok(results)
    }

fn parse_google_results(results: &Value) -> ParseOutput {
    let mut content = Vec::new();
    let mut links = Vec::new();

    if let Some(organic_results) = results.get("organic_results") {
        if let Some(organic_results_array) = organic_results.as_array() {
            for oresult in organic_results_array {
                if let Ok(oresult) = serde_json::from_value::<OrganicResult>(oresult.clone()) {
                    let temp = ParsedResult {
                        title: oresult.title,
                        date: oresult.date,
                        source: oresult.source,
                        content: oresult.snippet,
                        r#type: "search result snip".to_string(),
                    };
                    content.push(temp);
                    links.push(oresult.link);
                }
            }
        }
    }

    if let Some(related_questions) = results.get("related_questions") {
        if let Some(related_questions_array) = related_questions.as_array() {
            for oresult in related_questions_array {
                if let Ok(oresult) = serde_json::from_value::<RelatedQuestion>(oresult.clone()) {
                    let temp = ParsedResult {
                        title: oresult.title,
                        date: oresult.date,
                        source: None,
                        content: oresult.question,
                        r#type: "user comment".to_string(),
                    };
                    content.push(temp);
                }
            }
        }
    }

    ParseOutput { content, links }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <query>", args[0]);
        std::process::exit(1);
    }

    let query = &args[1];

    // Run Google query
    let results = search_query(query).await?;
    // Parse results
    let parsed_results = parse_google_results(&results);
    // Convert parsed results to a JSON string
    let json_string = serde_json::to_string(&parsed_results)?;

    // Define the file path
    let file_path = "data/results.json";

    // Write the JSON string to the file
    let mut file = File::create(file_path)?;
    file.write_all(json_string.as_bytes())?;

    println!("Parsed results have been written to {}", file_path);

    Ok(())
}