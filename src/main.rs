use std::path;
use structopt::StructOpt;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use failure::ResultExt;
use exitfailure::ExitFailure;

#[derive(Debug, StructOpt)]
#[structopt(name = "xd", about = "An toy implementation of Linux command xxd to view the content of a file in Hex.")]
struct Opt {
    #[structopt(short, long, help = "turn colorization on")]
    color_on: bool,

    #[structopt(short, long, default_value = "128", help = "stop after <length> bytes")]
    length: usize,

    #[structopt(parse(from_os_str))]
    input_file: path::PathBuf,

}
fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();
    println!("File name : {:#?}", opt.input_file);
    let f = File::open(opt.input_file)?;
    let mut f = BufReader::new(f);
    let mut buf = vec![];
    let mut line = [0u8; 16];
    let total_len = f.read_to_end(&mut buf)?;
    let mut cnt = 0;
    let mut idx = 0;
    for s in buf.bytes() {
        if cnt == 16 {
            print!("    ");
            for &i in line.iter() {
                if i >= 0x20 && i <= 0x7e { print!("{}", i as char) }
                else { print!(".") }
            }
            print!("\n");
            cnt = 0;
        } else if cnt == 8 {
            print!(" ");
        }
        if cnt == 0 {
            print!("{:08x}    ", idx);
            idx += 16;
        }
        let c = s.unwrap();
        line[cnt] = c;
        print!(" {:02x}", c);
        cnt += 1;
    }

    // print the remaing chars
    if cnt < 8 {
        print!("x");
    }
    for x in 0..(16-cnt) {
        print!("{:03}", x);
    }
    print!("    ");
    for i in 0..cnt {
        let i = line[i];
        if i >= 0x20 && i <= 0x7e { print!("{}", i as char) }
        else { print!(".") }
    }
    print!("\n{:08x}    ", idx-16+cnt);
    // println!("cnt = {:#?}", cnt);

    println!("\ntotal_len : {:#?} bytes", total_len);
    Ok(())
}
