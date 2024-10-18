use comix::{build_lines, convert_points, Geometry};

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The geometry collection to create
    #[arg(value_enum)]
    geometry: Geometry,

    /// The input file path
    #[arg(short, long, required = true)]
    input: PathBuf,

    /// The output file path
    #[arg(short, long, required = true)]
    output: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.geometry {
        Geometry::Point => {
            convert_points(cli.input, cli.output)?;
        }
        Geometry::Line => {
            build_lines(cli.input, cli.output)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
