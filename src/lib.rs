mod lines;
mod points;

use std::path::PathBuf;

use anyhow::Result;
use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Geometry {
    /// Point geometry in latitude and longitude coordinates
    Point,
    /// LineString geometry in latitude and longitude coordinates
    Line,
}

struct BuildArgs {
    pub input: PathBuf,
    pub output: PathBuf,
}

pub fn convert_points(input: PathBuf, output: PathBuf) -> Result<()> {
    let args = BuildArgs { input, output };
    points::write_points(args)
}

pub fn build_lines(input: PathBuf, output: PathBuf) -> Result<()> {
    let args = BuildArgs { input, output };
    lines::write_lines(args)
}
