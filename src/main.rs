use std::io::Write;
use std::path::Path;
use std::{fs, io::BufWriter, path::PathBuf};

use clap::Parser;
use color_eyre::eyre::Context;

/// Turns you code into a checkerboard pattern
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to directory or file
    path: PathBuf,

    /// Height of the checkerboard pattern
    height: u64,
}

fn checkerboard(path: &Path, height: u64) -> color_eyre::Result<()> {
    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read file: {path:?}"))?;

    let mut writer = BufWriter::new(fs::File::create(path)?);

    for (y, line) in content.lines().enumerate() {
        let line = line
            .chars()
            .enumerate()
            .map(|(x, c)| {
                if (x / (height as usize * 3) + y / height as usize) % 2 == 0 {
                    c
                } else {
                    ' '
                }
            })
            .collect::<String>();
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    for entry in ignore::Walk::new(args.path) {
        let entry = entry?;

        if entry.metadata()?.is_file() {
            if let Err(err) = checkerboard(entry.path(), args.height) {
                eprintln!("{err}");
            }
        }
    }

    Ok(())
}
