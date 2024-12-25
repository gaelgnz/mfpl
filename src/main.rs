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

    let mut string_vars: HashMap<String, &str> = HashMap::new();
    let mut int_vars: HashMap<String, i32> = HashMap::new();
    let mut bool_vars: HashMap<String, bool> = HashMap::new();

    for token in &contents {
        index += 1;
        if token.as_str().contains("PRINT") {
            let parts: Vec<&str> = token.split("/").collect();
            if parts.len() == 2 {
                if parts[1].contains("*") {
                    let subparts: Vec<&str> = parts[1].split("*").collect();
                    match subparts[0] {
                        "s" => print!("{}", match string_vars.get(subparts[1]) {
                            Some(string) => string,
                            None => {
                                println!("MFPL: Error: Couldnt find variable named {}", subparts[1]);
                                process::exit(1);
                            }
                        }),
                        "i" => print!("{}", match int_vars.get(subparts[1]) {
                            Some(int) => int,
                            None => {
                                println!("MFPL: Error: Couldnt find variable named {}", subparts[1]);
                                process::exit(1);
                            }
                        }),
                        &_ => {
                            println!("MFPL: Error: Unknown format specifier: {} at line {}", subparts[0], index)
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
            if parts.len() == 4 {
                match parts[2] {
                    "string" => {
                        string_vars.insert(parts[1].to_string(), parts[3]);
                    },
                    "int" => {
                        int_vars.insert(parts[1].to_string(), parts[3].parse().expect("MFPL: Error: Not an integer"));
                    },
                    "bool" => {
                        bool_vars.insert(parts[1].to_string(), match parts[3] {
                            "true" => true,
                            "false" => false,
                            &_ => {
                                println!("MFPL: Error: Not a bool");
                                process::exit(1);
                            }
                        });
                    }
                    &_ => {
                        println!("MFPL: Error: Unknown type {} at token {}", parts[2], index);
                        process::exit(1);
                    }
                }
            } else {
                println!("MFPL: SyntaxError: Not enough or too many arguments for VAR at line {}, expected 4 but {} was provided", index, parts.len());
                println!("MFPL: Note about ^: Parts are {:?}", parts);
                process::exit(1)
            }

        } else if token.as_str().contains("TERMINATE") {
            process::exit(0);

        } else if token.as_str().contains("ERRTERMINATE") {
            process::exit(1);
        }
    }
}
