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

async fn do_query<T: Provider>(query: &str, provider: &T, verbose: bool) -> Result<String> {
    let json = provider
        .do_query(query)
        .await?
        .json::<serde_json::Value>()
        .await?;

    if verbose {
        provider.get_details_from(&json);
    }

    provider.get_answer(&json)
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
    let answer = do_query(&query, &anthropic, args.verbose).await?;

    clipboard.set_text(&answer)?;

    println!();
    println!("`{}` copied into clipboard", answer.bold().green());

    Ok(())
}
