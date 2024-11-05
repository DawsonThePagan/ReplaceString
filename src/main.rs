use std::{env::{args, current_exe, set_current_dir}, fs::{self, copy, remove_file, File, OpenOptions}, io::{self, Read, Write}, path::Path, process};

const VER: &str = "1.0";

const ARG_FILE: &str = "/file";
const ARG_FROM: &str = "/from";
const ARG_TO: &str = "/to";
const ARG_NO_CASE: &str = "/nocase";
const ARG_HELP: &str = "/help";

const RET_OK: i32 = 0;
const RET_OK_TEMP_NO_DELETE: i32 = 1;
const RET_NOT_FOUND: i32 = 2;
const RET_ERR_IO: i32 = 3;
const RET_ERR_USER: i32 = 4;

const TEMP_FILE_POST: &str = ".temp";

fn main() {
    let args: Vec<String> = args().collect();
    let mut file: String = String::from("");
    let mut from: String = String::from("");
    let mut to: &str = "";
    let mut case = true;

    // Set working dir to location of EXE
    match current_exe() {
        Ok(path) => {
            let path_form = match path.parent() {
                None => {
                    println!("Error while initialising, could not set working dir");
                    process::exit(RET_ERR_IO);
                }
                Some(d) => d
            };

            match set_current_dir(&path_form) {
                Ok(_) => {
                    println!("=== ReplaceString Starting V{VER}===");
                    println!();
                }

                Err(_) => {
                    println!("Error while initialising, could not set working dir to {}", path_form.display());
                    process::exit(RET_ERR_IO);
                }
            };
        }
        Err(e) => {
            println!("Error while initialising, could not set working dir {e}");
            process::exit(RET_ERR_IO);
        }
    }
    
    // Read arguments
    if args.len() == 1 {
        println!("Arguments not set correctly");
        process::exit(RET_ERR_USER);
    }

    if args[1] == ARG_HELP {
        help();
        pause();
        process::exit(RET_OK);
    }

    let mut arg_n: usize = 1;
    loop {
        if arg_n >= args.len() {
            break;
        }

        if args[arg_n].to_lowercase() == ARG_FILE {
            if args[arg_n + 1].starts_with(".\\") || args[arg_n + 1].contains(":\\") {
                file = args[arg_n + 1].clone();
            }else{
                file = ".\\".to_string() + args[arg_n + 1].as_str();
            }
            
            arg_n = arg_n + 1;
        }

        if args[arg_n].to_lowercase() == ARG_FROM {
            from = args[arg_n + 1].clone();
            arg_n = arg_n + 1;
        }

        if args[arg_n].to_lowercase() == ARG_TO {
            to = args[arg_n + 1].as_str();
            arg_n = arg_n + 1;
        }

        if args[arg_n].to_lowercase() == ARG_NO_CASE {
            case = false;
        }

        arg_n = arg_n + 1;
    }

    // Check the arguments are good
    if file == "" || from == "" || to == "" {
        println!("Arguments not set correctly");
        process::exit(RET_ERR_USER);
    }

    // Remove case
    if !case {
        from = from.to_lowercase();
    }

    println!("Checking if {file} exists");
    if !Path::new(&file).is_file() {
        println!("Path provided does not exist");
        process::exit(RET_ERR_USER);
    }

    // Prepare needed variables
    let temp_file: String = file.clone() + TEMP_FILE_POST;
    let mut changed:bool = false;
    let mut temp_out  = match OpenOptions::new().create(true).write(true).open(&temp_file) {
        Ok(x) => x,
        Err(e) => {
            println!("Could not open temp file {e}");
            process::exit(RET_ERR_IO)
        }
    }; 

    let mut file_handle = match File::open(&file){
        Ok(d) => d,
        Err(e) => {
            println!("Failed to read file. {e}");
            process::exit(RET_ERR_IO);
        }
    };

    let mut file_data = String::new();
    _ = match file_handle.read_to_string(&mut file_data) {
        Ok(d) => d,
        Err(e) => {
            println!("Failed to read file. {e}");
            process::exit(RET_ERR_IO);
        }
    };

    // Loop through original file
    for line in file_data.lines() {
        let mut line_check: String = String::from(line);
        if !case {
            line_check = line_check.to_lowercase();
        }

        let line_edit;
        if line_check.contains(&from) { // Find what needs to be changed
            line_edit = line_check.replace(&from, to); // Try to change it
            
            // Above returns the output of the string after replacing requested. 
            // We can check if it worked by checking if its same as the original
            if line_edit == line_check { 
                println!("Failed to replace text, unknown error");
                drop(temp_out);
                // Delete the file, doesn't matter if this works
                _ = fs::remove_file(temp_file);
                process::exit(RET_ERR_IO);
            }

            changed = true;
        }
        else{ // If its not there continue as normal
            line_edit = line.to_string(); 
        }

        // Write that to the buffer
        match temp_out.write_all(line_edit.as_bytes()) {
            Err(_) => {
                println!("Failed to write text to temp file");
                drop(temp_out);
                // Delete the file, doesn't matter if this works
                _ = remove_file(temp_file);
                process::exit(RET_ERR_IO);
            }

            Ok(_) => {
                // If the write to buffer works then flush that to file
                match temp_out.flush() {
                    Ok(_) => {
                        continue;
                    }
                    Err(_) => {
                        println!("Failed to flush text to temp");
                        drop(temp_out);
                        // Delete the file, doesn't matter if this works
                        _ = remove_file(temp_file);
                        process::exit(RET_ERR_IO);
                    }
                };
            }
        };
    }

    // If we didn't find what was wanted to change then inform the user
    if !changed {
        println!("Could not find text requested to be changed");
        drop(temp_out);
        // Delete the file, doesn't matter if this works
        _ = remove_file(temp_file);
        process::exit(RET_NOT_FOUND);
    }

    drop(temp_out);

    // Overwrite the old file with the new file
    match copy(&temp_file, file) {
        Ok(_) => {
            // Try to delete the temp file
            match remove_file(&temp_file) {
                Ok(_) => {
                    println!("Successfully replaced text in file");
                    process::exit(RET_OK);
                }

                Err(_) => {
                    // Even if we fail to delete the temp file we still succeeded
                    println!("Successfully replaced text in file, temp file failed to delete");
                    process::exit(RET_OK_TEMP_NO_DELETE);
                }   
            }
        }

        Err(_) => {
            // Don't delete the temp file in this case as this could be salvaged externally
            println!("Failed to overwrite original file, temp file still exists");
            process::exit(RET_ERR_IO);
        }   
    }
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn help() {
    println!("Replace a string in a file without using powershell");
    println!();
    println!("Return codes:");
    println!(" 0 = Ok replaced text correctly");
    println!(" 1 = Ok replaced text correctly. Failed to delete temp file");
    println!(" 2 = Text asked to change was not found");
    println!(" 3 = IO error");
    println!(" 4 = Arguments provided were incorrect");
    println!();
    println!("Arguments:");
    println!("/file = Required, file to edit");
    println!("/from = Required, text to change from. See /nocase for case sensitivity");
    println!("/to = Required, text to change to");
    println!("/nocase = Optional, by default from in case sensitive, this disables that");
    println!();
}