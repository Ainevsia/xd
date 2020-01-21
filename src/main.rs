use exitfailure::ExitFailure;
use xd::*;

fn main() -> Result<(), ExitFailure> {
    // phare the argument and figure out the filename
    let opt = Opt::new();
    // println!("opt = {:#?}", opt);

    let file_name = opt.get_file_name();
    println!("File name : {:#?}", file_name);

    // get the content and print
    let mut buf = vec![];
    get_content(file_name, &mut buf)?;
    match opt.get_color_option() {
        true => print_hex_color(&buf, opt.get_length()),
        false => print_hex(&buf, opt.get_length()),
    }
    

    Ok(())
}
