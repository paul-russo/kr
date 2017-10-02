#![feature(conservative_impl_trait)]

extern crate terminal_size;

use std::fs;
use std::env;
use std::os::unix::fs::PermissionsExt;

use terminal_size::{Width, Height, terminal_size};

fn char_at(pos: usize) -> impl Fn(&String) -> String {
    move |some_string| some_string.chars().nth(pos).unwrap().to_string()
}

fn add_path_string(base: &String, addition: &String) -> String {
    format!("{}        {}", base, addition)
}

fn print_lines(lines: Vec<String>) {
    for line in lines {
        println!("{}", line);
    }
}

fn display_list(path_list: fs::ReadDir, show_all: bool, show_metadata: bool, compact: bool) {
    // Get terminal width
    let terminal_width = match terminal_size() {
        None => 100,
        Some((Width(w), Height(_))) => w,
    };

    let mut lines = vec![String::new()];

    for path in path_list {
        let the_path = path.unwrap();
        let metadata = the_path.metadata().unwrap();

        let path_string = match the_path.file_name().into_string() {
            Ok(bare_path) => bare_path,
            Err(_) => String::new(),
        };

        let is_hidden = char_at(0)(&path_string) == ".";

        // If we're showing hidden paths, or if the path is not hidden...
        if show_all || !is_hidden {
            let lines_length = lines.len();

            // If the current line has no length...
            if lines[lines_length - 1].len() == 0 {
                // Set the current line equal to the path string
                lines[lines_length - 1] = path_string;
            } else {
                if compact {
                    // If we're trying to display the paths on as few lines as possible
                    if (path_string.len() + 8 + lines[lines_length - 1].len()) > (terminal_width as usize) {
                        lines.push(path_string);
                    } else {
                        lines[lines_length - 1] = add_path_string(&lines[lines_length - 1], &path_string);
                    }
                } else {
                    // Otherwise, put each path on its own line
                    lines.push(path_string);
                }
            }
        }
    }

    print_lines(lines);
}

fn main() {
    let flags: Vec<String> = env::args().filter(|arg| arg.split("-").nth(0) == Some("")).collect();
    let path = env::args().filter(|arg| arg.split("-").nth(0) != Some("")).nth(1);

    // Get listing path. Defaults to "./"
    let listing_path = match path {
        None => String::from("./"),
        Some(arg_path) => arg_path.trim().to_string(),
    };

    let show_all = match flags.iter().find(|flag| flag == &"-a") {
        None => false,
        Some(_) => true,
    };

    let show_metadata = match flags.iter().find(|flag| flag == &"-l") {
        None => false,
        Some(_) => true,
    };

    let compact = !show_metadata;

    // Get a list of paths from the listing path
    match fs::read_dir(listing_path) {
        Ok(listing_result) => display_list(listing_result, show_all, show_metadata, compact),
        Err(err) => println!("Error: {}", err),
    }
}
