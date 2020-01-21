use exitfailure::ExitFailure;
use xd::*;

fn main() -> Result<(), ExitFailure> {
    // phare the argument and figure out the filename
    let opt = Opt::new();
    let file_name = opt.get_file_name();
    println!("File name : {:#?}", file_name);

    let mut buf = vec![];
    let mut line = [0u8; 16];
    let total_len = get_content(file_name, &mut buf)?;
    let (mut cnt, mut idx) = (0, 0);
    for byte in buf.into_iter() {
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
        let c = byte;
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
