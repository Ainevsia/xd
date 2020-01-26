use std::path;
use structopt::StructOpt;
use exitfailure::ExitFailure;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use colored::*;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "xd",
    about = "An toy implementation of Linux command xxd to view the content of a file in Hex.",
    author = "Ainevsia")]
pub struct Opt {
    #[structopt(short, long, help = "turn colorization on")]
    color_on: bool,

    #[structopt(short, long, help = "stop after <length> bytes")]
    length: Option<usize>,

    #[structopt(name = "filename", parse(from_os_str))]
    input_file: path::PathBuf,

}

impl Opt {
    pub fn new() -> Opt {
        Opt::from_args()
    }

    pub fn get_file_name(&self) -> &path::Path {
        &self.input_file
    }

    pub fn get_length(&self) -> Option<usize> {
        self.length
    }

    pub fn get_color_option(&self) -> bool {
        self.color_on
    }
}

pub fn open_reader(file_name: &path::Path) -> Result<BufReader<File>, ExitFailure>
{
    Ok(
        BufReader::new(
            File::open(file_name)?
        )
    )
}

pub fn print_hex(reader: impl Read, len: Option<usize>, color: bool) -> Result<(), ExitFailure> {
    let mut reader: Box<dyn Read> = match len {
        Some(len) => Box::new(reader.take(len as u64)),
        None => Box::new(reader),
    };
    let mut buf = [0u8; 16];
    let mut printed = 0;

    loop {
        let len = reader.read(&mut buf)?;
        if len == 0 {
            break;
        }

        // start a new line
        if color {
            print!("{}", format!("{:08x}    ", printed).cyan());
        } else {
            print!("{:08x}    ", printed);
        }
        printed += len;

        for i in 0..16 {
            if i == 8 { // middle of the hex, add one blank space
                print!(" ");
            }
            if i < len {
                // 2-digit hex
                if color {
                    print!("{}", format!(" {:02x}", buf[i]).green());
                } else {
                    print!(" {:02x}", buf[i]);
                }
            } else {
                print!("   "); // placeholder
            }
        }

        // one line of hex end, print corresponding chars
        print!("    ");
        for &i in &buf[0..len] {
            let ascii = if i >= 0x20 && i <= 0x7e { i as char } else { '.' };
            if color {
                print!("{}", format!("{}", ascii).blue());
            } else {
                print!("{}", ascii);
            }
        }
        println!();
    }

    // finish summary
    if color {
        println!("{}", format!("{:08x}", printed).cyan());
    } else {
        println!("{:08x}", printed);
    }
    println!("total length : {:#?} bytes", printed);

    Ok(())
}
