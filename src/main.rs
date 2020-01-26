use exitfailure::ExitFailure;
use xd::*;

fn main() -> Result<(), ExitFailure> {
    // parse the argument and figure out the filename
    let opt = Opt::new();
    // println!("opt = {:#?}", opt);

    let file_name = opt.get_file_name();
    println!("File name : {:#?}", file_name);

    // get the content and print
    let reader = open_reader(file_name)?;
    print_hex(reader, opt.get_length(), opt.get_color_option())?;

    Ok(())
}
