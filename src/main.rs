use std::str::FromStr;

#[derive(Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn get_args() -> Result<Cli, Box<dyn std::error::Error>> {
    let pattern = std::env::args().nth(1).ok_or_else(|| "no pattern given")?;
    let path = std::env::args().nth(2).ok_or_else(|| "no path given")?;
    let path = std::path::PathBuf::from_str(&path)?;
    Ok(Cli { pattern, path })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = get_args()?;

    dbg!(cli);
    println!("Hello, world!");

    Ok(())
}
