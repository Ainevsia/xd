use std::path;
use structopt::StructOpt;
use exitfailure::ExitFailure;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug, StructOpt)]
#[structopt(name = "xd", about = "An toy implementation of Linux command xxd to view the content of a file in Hex.")]
pub struct Opt {
    #[structopt(short, long, help = "turn colorization on")]
    color_on: bool,

    #[structopt(short, long, default_value = "128", help = "stop after <length> bytes")]
    length: usize,

    #[structopt(parse(from_os_str))]
    input_file: path::PathBuf,

}

impl Opt {
    pub fn new() -> Opt {
        Opt::from_args()
    }

    pub fn get_file_name(&self) -> &path::Path {
        &self.input_file
    }
}

pub fn get_content(file_name: &path::Path, buf: &mut Vec<u8>) -> Result<usize, ExitFailure>
{
    Ok(
        BufReader::new(
            File::open(file_name)?
        ).read_to_end(buf)?
    )
}