use std::env;
use std::fs::{self, OpenOptions};
use std::collections::HashMap;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        2 => println!("MFPL: Opening file {}", &args[1]),
        _ => {
            eprintln!("MFPL: Provide 1 argument");
            process::exit(1);
        }
    }

    let filename: &String = &args[1];

    let _file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&filename) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("MFPL: Error opening file: {}", error);
                process::exit(1);
            }
        };

    let contents = match fs::read_to_string(&filename) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("MFPL: Error reading file: {}", error);
            process::exit(1);
        }
    };
    let contents: Vec<String> = contents.split(";").map(String::from).collect();

    println!("MFPL: running");

    // RUNNING PROGRAM (INTERPRETED)

    let mut index: i32 = -1;

    let mut vars: HashMap<String, &str> = HashMap::new();

    for token in &contents {
        index += 1;
        if token.as_str().contains("PRINT") {
            let parts: Vec<&str> = token.split("/").collect();
            if parts.len() == 2 {
                if parts[1].contains("*") {
                    match vars.get(parts[1].trim_start_matches('*')) {
                        Some(value) => print!("{}", value),
                        None => {
                            println!("MFPL: Error: Invalid variable: {}", parts[1]);
                            process::exit(1);
                        }
                    }
                } else {
                    print!("{}", parts[1]);
                }
            } else {
                eprintln!("MFPL: Error: PRINT command format is incorrect at token number: {}", index);
                process::exit(1);
            }

        } else if token.as_str().contains("VAR") {
            let parts: Vec<&str> = token.split("/").collect();
            if parts.len() == 3 {
                vars.insert(parts[1].to_string(), parts[2]);
            }

        } else if token.as_str().contains("SHOWVARS") {
            for (key, value) in &vars {
                println!("{}: {}", key, value);
            }

        } else if token.as_str().contains("TERMINATE") {
            process::exit(0);

        } else if token.as_str().contains("ERRTERMINATE") {
            process::exit(1);
        }
        // match token.as_str() {
        //     "PRINT" => {
        //         if index as usize + 1 < contents.len() {
        //             print!("{}", &contents[index as usize + 1]);
        //         } else {
        //             eprintln!("MFPL: Error: PRINT command without argument at token number: {}", index as usize);
        //             process::exit(1);
        //         }
        //     }
        //     &_ => {
        //         println!("MFPL: Error: Unexpected unknown token: {}, On token number: {}", &contents[index as usize], index as usize);
        //         process::exit(1);
        //     }
        // }
    }
}
