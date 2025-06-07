mod errors;
mod llm;

use arboard::Clipboard;
use clap::Parser;
use errors::AskError;
use indicatif::ProgressBar;
use inquire::{Text, validator::ValueRequiredValidator};
use llm::{Provider, anthropic::Anthropic};
use owo_colors::OwoColorize;
use std::{env, process::ExitCode, time::Duration};
use tokio::{sync::watch, time::sleep};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    query: Option<String>,

    #[arg(short, long)]
    verbose: bool,
}

fn get_query(query: Option<String>) -> Result<String, AskError> {
    let validator = ValueRequiredValidator::new("A query is required");
    let query = match query {
        Some(query) => query,
        None => Text::new("").with_validator(validator).prompt()?,
    };

    Ok(query)
}

async fn do_query<T: Provider>(
    query: &str,
    provider: &T,
    verbose: bool,
) -> Result<String, AskError> {
    let json = provider.do_query(query).await?;

    if verbose {
        provider.get_details_from(&json);
    }

    provider.get_answer_from(&json)
}

async fn run_spinner(mut shutdown_rx: watch::Receiver<bool>) {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_message(" Thinking...");

    loop {
        tokio::select! {
            Ok(_) = shutdown_rx.changed() => {
                if *shutdown_rx.borrow() {
                    break;
                }
            }
            _ = sleep(Duration::from_millis(100)) => {
                if !pb.is_finished() {
                    pb.tick();
                } else {
                    break;
                }
            }
        }
    }

    if !pb.is_finished() {
        pb.finish_and_clear();
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{} {}", "Error: ".red().bold(), e);
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), AskError> {
    let Ok(api_key) = env::var(Anthropic::API_KEY_ENV) else {
        return Err(AskError::KeyMissing(Anthropic::API_KEY_ENV.to_string()));
    };

    let args = Args::parse();
    let anthropic = Anthropic::new(&api_key);
    let query = get_query(args.query)?;

    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let spinner_handle = tokio::spawn(async move { run_spinner(shutdown_rx).await });

    let answer = do_query(&query, &anthropic, args.verbose).await?;

    if shutdown_tx.send(true).is_err() {
        eprintln!("Spinner task was already gone before shutdown signal.");
    }

    if let Err(e) = spinner_handle.await {
        eprintln!("Spinner task panicked: {:?}", e);
    }

    Clipboard::new()?.set_text(&answer)?;

    println!();
    println!("`{}` copied into clipboard", answer.bold().green());

    Ok(())
}
