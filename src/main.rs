mod llm;

use anyhow::{Result, bail};
use arboard::Clipboard;
use clap::Parser;
use inquire::{Text, validator::ValueRequiredValidator};
use llm::{Provider, anthropic::Anthropic};
use owo_colors::OwoColorize;
use std::env;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    query: Option<String>,

    #[arg(short, long)]
    verbose: bool,
}

fn get_query(query: Option<String>) -> Result<String> {
    let validator = ValueRequiredValidator::new("A query is required");
    let query = match query {
        Some(query) => query,
        None => Text::new("").with_validator(validator).prompt()?,
    };

    Ok(query)
}

#[tokio::main]
async fn main() -> Result<()> {
    let Ok(api_key) = env::var(Anthropic::API_KEY_ENV) else {
        bail!("Please provide a valid {}", Anthropic::API_KEY_ENV);
    };

    let args = Args::parse();
    let mut clipboard = Clipboard::new()?;
    let anthropic = Anthropic::new(&api_key);

    let query = get_query(args.query)?;
    let json: serde_json::Value = anthropic.do_query(&query).await?.json().await?;

    if args.verbose {
        anthropic.get_details_from(&json);
    }

    let answer = anthropic.get_answer(&json)?;
    clipboard.set_text(&answer)?;

    println!();
    println!("`{}` copied into clipboard", answer.bold().green());

    Ok(())
}
