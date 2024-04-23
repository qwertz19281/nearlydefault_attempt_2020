use std::path::{PathBuf, Path};
use structopt::*;
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let o = ::medion::Opt::from_args();

    ::medion::run(o)
}
