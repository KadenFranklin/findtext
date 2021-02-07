use std::{env, io, fs};


fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let pattern = &args[1];
    let args = env::args().skip(1);

    for file in args.skip(1) {
        let file_contents = fs::read_to_string(&file)?;
        if file_contents.contains(pattern){
            for lines in file_contents.lines() {
                if lines.contains(pattern) {
                    println!("{}", lines);
                }
            }
        }
    }
    Ok(())
}