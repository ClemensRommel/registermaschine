mod vm;
mod compiler;

use std::{fs::File, env, io::Read};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 { // Illegal argument count
        println!("Error: Expected 1 argument");
    } else {
        let file = File::open(&args[1]); // Open the specified file
        if let Result::Ok(mut file) = file { // File was opened
            // Read the file
            let mut content = String::new();
            file.read_to_string(&mut content).expect("Error: Could not read file");

            // Compile the source
            let vm = compiler::compile(&content);
            
            // If compilation was successful
            if let Some(mut vm) = vm {
                vm.run();
            } else {
                println!("Error while compiling vm");
            }
        } else if let Result::Err(err) = file {
            println!("{}", err);
        }
    }
}


