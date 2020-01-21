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

pub fn print_hex(buf: &Vec<u8>) {
    let (mut cnt, mut idx) = (0, 0);
    let mut line = [0u8; 16];

    for &byte in buf.iter() {
        if cnt == 16 {  // one line of hex end, print corresponding chars
            print!("    ");
            for &i in line.iter() {
                if i >= 0x20 && i <= 0x7e { print!("{}", i as char) }
                else { print!(".") }
            }
            print!("\n");
            cnt = 0;
        } else if cnt == 8 {    // middle of the hex, add one blank space
            print!(" ");
        }
        if cnt == 0 {   // start a new line
            print!("{:08x}    ", idx);
            idx += 16;
        }
        line[cnt] = byte;
        print!(" {:02x}", byte);
        cnt += 1;
    }

    // print the remaing chars
    if cnt < 8 { print!(" ") }
    for _ in 0..16 - cnt { print!("   ") }
    print!("    ");
    for i in 0..cnt {
        let c = line[i];
        if c >= 0x20 && c <= 0x7e { print!("{}", c as char) }
        else { print!(".") }
    }

    // finish summary
    let length = idx - 16 + cnt;
    println!("\n{:08x}", length);
    println!("total length : {:#?} bytes", length);
}