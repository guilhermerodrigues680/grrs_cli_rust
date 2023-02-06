use anyhow::Context;
use clap::Parser;
use log::info;
use log::trace;
use std::io::BufRead;
use std::io::BufReader;

// #[derive(Debug)]
// struct CustomError(String);

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    trace!("Fazendo parse dos argumentos");
    let args = Cli::parse();

    trace!("Abrindo arquivo");
    let file = std::fs::File::open(&args.path).with_context(|| {
        format!(
            "Error reading `{}`",
            &args.path.to_str().unwrap_or_default()
        )
    })?;
    // let file = match std::fs::File::open(&args.path) {
    //     Ok(f) => f,
    //     Err(error) => Err(format!(
    //         "Não é possível lidar com `{error}`, encerrando programa."
    //     ))?,
    // };
    // let file = std::fs::File::open(&args.path).map_err(|err| {
    //     CustomError(format!(
    //         "Error reading `{}`: {}",
    //         &args.path.to_str().unwrap_or_default(),
    //         err
    //     ))
    // })?;

    trace!("Criando buffer e lendo arquivo linha a linha");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.contains(&args.pattern) {
            println!("{line}");
        }
    }

    info!("Programa encerrado com sucesso.");
    Ok(())
}
