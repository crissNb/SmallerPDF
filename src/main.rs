use std::path::Path;
use std::process::Command;
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Provide a file path!");
        exit(0);
    }

    // Check if file exists and is pdf
    let file_path = Path::new(&args[1]);
    if !file_path.exists() {
        println!("File does not exist!");
        exit(0);
    }

    // TODO: Ability to optimize an entire folder of documents, recursively
    if file_path.extension().unwrap() != "pdf" {
        println!("Provided file is not a PDF document!");
        exit(0);
    }

    let mut gs_command = Command::new("gs");
    gs_command
        .arg("-dBATCH")
        .arg("-dNOPAUSE")
        .arg("-q")
        .arg("-dCompatibilityLevel=1.4")
        .arg("-dPDFSETTINGS=/screen")
        .arg("-r72")
        .arg("-sDEVICE=pdfwrite")
        .arg("-sOutputFile=output.pdf")
        .arg(&file_path);

    Command::output(&mut gs_command).expect("Failure");

    println!("Result saved in output.pdf!");
}
