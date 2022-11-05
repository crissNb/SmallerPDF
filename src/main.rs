use std::{env, ffi::OsStr, path::PathBuf, process::Command};

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

    if let Err(e) = Command::new("gs")
        .arg("-dBATCH")
        .arg("-dNOPAUSE")
        .arg("-q")
        .arg("-dCompatibilityLevel=1.4")
        .arg("-dPDFSETTINGS=/screen")
        .arg("-r72")
        .arg("-sDEVICE=pdfwrite")
        .arg("-sOutputFile=output.pdf")
        .arg(&file_path)
        .output()
    {
        println!("Unable to run GhostScript! Error: '{e}'");
        return;
    }

    println!("Result saved in output.pdf!");
}
