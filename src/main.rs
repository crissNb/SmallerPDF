use std::{env, ffi::OsStr, path::{PathBuf, Path}, process::Command};

fn main() {
    let Some(file_path): Option<PathBuf> = env::args().nth(1).map(Into::into) else {
        println!("Provide a file path!");
        return;
    };

    // Check if file exists and can be read.
    match file_path.try_exists() {
        Ok(true) => (),
        Ok(false) => {
            println!("File does not exist!");
            return;
        }
        Err(e) => {
            println!("Can't access file! Error: '{e}'");
            return;
        }
    }

    // TODO: Ability to optimize an entire folder of documents, recursively

    // Check if file is pdf.
    if file_path.extension() != Some(OsStr::new("pdf")) {
        println!("Provided file is not a PDF document!");
        return;
    }

    let mut _image_resolution = 72;
    if env::args().len() > 2 {
        let _number: i32 = match env::args().nth(2).unwrap().parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Provide an integer to adjust image resolution!");
                return;
            }
        };

        _image_resolution = _number;
    }

    let _before_size = file_path.metadata().unwrap().len() as f32;

    if let Err(e) = Command::new("gs")
        .arg("-dBATCH")
        .arg("-dNOPAUSE")
        .arg("-q")
        .arg("-dCompatibilityLevel=1.4")
        .arg("-dPDFSETTINGS=/screen")
        .arg(format!("-r{}", _image_resolution))
        .arg("-sDEVICE=pdfwrite")
        .arg("-sOutputFile=output.pdf")
        .arg(&file_path)
        .output()
    {
        println!("Unable to run GhostScript! Error: '{e}'");
        return;
    }

    println!("Result saved in output.pdf!");

    let _output_path = Path::new("output.pdf");
    let _after_size = _output_path.metadata().unwrap().len() as f32;

    println!("File size changes: {:.2}MB -> {:.2}MB", _before_size / 1000000.0, _after_size / 1000000.0);
}
