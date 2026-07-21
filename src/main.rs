use rig::{completion::Prompt, prelude::*, providers::gemini};
use std::env;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Ensure the API key is present
    let api_key = env::var("GEMINI_API_KEY")
        .expect("Error: Please set the GEMINI_API_KEY environment variable.");

    // 2. Parse the target file path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run -- <path_to_file>");
        println!("Example: cargo run -- document.txt");
        return Ok(());
    }
    let file_path = &args[1];

    // 3. Verify the file exists and read it
    if !Path::new(file_path).exists() {
        println!("Error: The file '{}' does not exist.", file_path);
        return Ok(());
    }

    println!("Reading file: {}...", file_path);
    let input_text = fs::read_to_string(file_path)?;

    if input_text.trim().is_empty() {
        println!("Error: The file is empty.");
        return Ok(());
    }

    // 4. Initialize Gemini client and Agent
    let client = gemini::Client::new(&api_key);
    let summarizer_agent = client.unwrap()
        .agent("gemini-3.5-flash") 
        .preamble("You are an expert AI text summarizer. Extract the core points. Provide a brief overview, followed by 3-10 punchy bullet points. Be concise.")
        .build();

    println!("Generating summary... Please wait.\n");

    // 5. Query the model
    let prompt_message = format!("Please summarize this text file:\n\n{}", input_text);
    let summary_output = summarizer_agent.prompt(&prompt_message).await?;

    // 6. Print results
    println!("========= AI SUMMARY OF: {} =========", file_path);
    println!("{}", summary_output);
    println!("==================================================");

    Ok(())
}
