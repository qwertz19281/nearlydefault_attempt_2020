use std::{sync::Arc, path::{PathBuf, Path}};
use structopt::*;
use regex::Regex;

pub mod iter;
pub mod alg;
pub mod alg_alt;

#[derive(StructOpt)]
#[structopt(name = "medion", about = "Median Filter processor recursive NearlyDefault algorithm")]
pub struct Opt {
    #[structopt(short,long,default_value="2",help="scale; 1+")]
    pub scale: u32,
    #[structopt(short,long,default_value="1",help="factor; 1+")]
    pub factor: u32,

    #[structopt(short,long)]
    pub wrap: bool,

    #[structopt(short,long,default_value="0.0")]
    pub circular_mod: f32,

    #[structopt(long)]
    pub alternative: bool,

    #[structopt(short,long,help="if set, only matching names (regex) will be processed")]
    pub include: Option<String>,

    #[structopt(short="x",long,help="exclude matching names (regex)")]
    pub exclude: Option<String>,

    #[structopt(parse(from_os_str),required=true)]
    pub input: PathBuf,

    #[structopt(parse(from_os_str),required=true)]
    pub output: PathBuf,

    pub include_regex: Option<Regex>,
    pub exclude_regex: Option<Regex>,
}

pub fn run(mut o: Opt) -> anyhow::Result<()> {
    anyhow::ensure!(o.scale != 0,"Error: scale must not be 0");
    anyhow::ensure!(o.factor != 0,"Error: factor must not be 0");

    anyhow::ensure!(o.input.is_dir(),"Error: Input is not an existing directory");
    anyhow::ensure!(!o.output.is_file(),"Error:  Output is a file");

    o.include_regex =
        if let Some(x) = &o.include {
            Some( Regex::new(x)? )
        }else{
            None
        };
    o.exclude_regex =
        if let Some(x) = &o.exclude {
            Some( Regex::new(x)? )
        }else{
            None
        };

    let mut pool = rayon::ThreadPoolBuilder::new().build().unwrap();

    let o = Arc::new(o);

    pool.scope(|p| {
        iter::transfer_dir(&o.input, &o.output, p, &o)
    })
}

pub type Image = image::RgbaImage;

#[macro_export]
macro_rules! soft_error {
    ($oof:expr,$fmt:expr) => {
        match $oof {
            Ok(f) => {
                Some(f)
            },
            Err(e) => {
                eprintln!($fmt,e);
                None
            },
        }
    };
}
