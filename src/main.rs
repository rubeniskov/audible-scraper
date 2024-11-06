use audible_scrapper::{AudioBook, Builder, QueryParams};
use clap::{Parser, ValueEnum};
use csv;
use serde_json;
use std::io::Write;
use toml;

/// CLI to scrape Audible audiobooks
#[derive(Parser)]
struct Args {
    /// Narrator name to filter audiobooks (optional)
    #[arg(short, long)]
    narrator: Option<String>,

    #[arg(short, long)]
    keywords: Option<String>,

    /// Output format: jsonl, csv, json, or toml
    #[arg(short, long, value_enum, default_value = "json")]
    format: OutputFormat,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
    Jsonl,
    Csv,
    Json,
    Toml,
}

#[tokio::main]
async fn main() {
    // Parse the command-line arguments
    let args = Args::parse();

    // Configura los parámetros de búsqueda, usando narrator solo si está presente
    let mut params = QueryParams::new();

    if let Some(narrator) = args.narrator.as_deref() {
        params = params.narrator(narrator);
    }

    if let Some(keywords) = args.keywords.as_deref() {
        params = params.keywords(keywords);
    }

    let scrapper = Builder::new(params)
        .build()
        .expect("Failed to build scrapper");

    match scrapper.fetch_all().await {
        Ok(result) => {
            let audiobooks: Vec<AudioBook> =
                result.iter().flat_map(|f| f.collect().unwrap()).collect();

            // Handle output format based on the format parameter
            match args.format {
                OutputFormat::Jsonl => {
                    for audiobook in &audiobooks {
                        std::io::stdout()
                            .write_all(serde_json::to_string(&audiobook).unwrap().as_bytes())
                            .unwrap();
                        std::io::stdout().write_all(b"\n").unwrap();
                    }
                }
                OutputFormat::Csv => {
                    let mut wtr = csv::Writer::from_writer(std::io::stdout());
                    for audiobook in &audiobooks {
                        wtr.serialize(audiobook).unwrap();
                    }
                    wtr.flush().unwrap();
                }
                OutputFormat::Json => {
                    std::io::stdout()
                        .write_all(
                            serde_json::to_string_pretty(&audiobooks)
                                .unwrap()
                                .as_bytes(),
                        )
                        .unwrap();
                }
                OutputFormat::Toml => match toml::to_string(&audiobooks) {
                    Ok(toml_output) => {
                        std::io::stdout().write_all(toml_output.as_bytes()).unwrap();
                    }
                    Err(e) => {
                        eprintln!("Error serializing to TOML: {:?}", e);
                    }
                },
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
