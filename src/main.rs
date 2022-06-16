use std::io::Write;
use std::fs::{File, OpenOptions};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut floc: String = "test.rs".to_string();
    let mut cuse: String = String::new();

    if args.len() >= 2{
        if !args[1].is_empty() {
            floc = args[1].clone();
        }
        if args.len() >= 3 {
            let mut i: usize = 2;
            loop {
                cuse = cuse + "use " + &args[i] + ";" + "\n";
                i += 1;
                if args.len() <= i {
                    break;
                }
            }
            cuse += "\n";
        }
    }
    
    let sstr: String = cuse + "fn main(){\n\n}";
    let mut fp: File = OpenOptions::new().append(false).create(true).read(true).write(true).open(&floc).expect("Failed to open File");
    fp.write(&sstr.into_bytes()).expect("Failed to write to File");

    print!("Success!\n");
}