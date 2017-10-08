#![feature(conservative_impl_trait)]

extern crate terminal_size;

use std::io;
use std::fs;
use std::ffi;
use std::env;
use std::cmp;
use std::os::unix::fs::PermissionsExt;

use terminal_size::{Width, Height, terminal_size};

// Helpers
fn char_at(pos: usize) -> impl Fn(&String) -> String {
    move |some_string| some_string.chars().nth(pos).unwrap().to_string()
}

fn pad_right(base: String, padded_length: usize) -> String {
    let gap = padded_length - base.len();
    let pad = std::iter::repeat(" ").take(gap).collect::<String>();
    format!("{}{}", base, pad)
}

fn string_from_file_name(file_name: ffi::OsString) -> String {
    match file_name.into_string() {
        Ok(path) => path,
        Err(_) => String::new(),
    }
}

// End helpers

fn print_lines(lines: Vec<Vec<String>>, col_length: usize) {
    for line in lines {
        let formatted_line: String = line.into_iter().fold(String::new(), |l, w| {
            format!("{}{}", l, pad_right(w, col_length))
        });

        println!("{}", formatted_line);
    }
}

fn display_list(path_list: fs::ReadDir, show_all: bool, long: bool) {
    let path_vec: Vec<io::Result<fs::DirEntry>> = path_list.collect();

    // Get vector of tuples, each of which will contain a stringified pathname, and an fs::Metadata struct
    let dirs: Vec<(String, fs::Metadata)> = path_vec.into_iter().filter_map(|path| {
        match path {
            Ok(dir) => Some((string_from_file_name(dir.file_name()), dir.metadata().unwrap())),
            Err(_) => None,
        }
    }).collect();

    let filtered_dirs: Vec<(String, fs::Metadata)> = match show_all {
        true => dirs.clone(),
        false => dirs.into_iter().filter(|dir| char_at(0)(&dir.0) != ".").collect()
    };

    // Calculate the column length, by getting the maximum path length and adding 1 (a space) to it
    let col_length: usize = filtered_dirs.iter().fold(0, |max, dir| cmp::max(dir.0.len(), max)) + 1;

    // Get terminal width
    let terminal_width = match terminal_size() {
        None => 100,
        Some((Width(w), Height(_))) => w,
    };

    let cols_per_row = cmp::max((terminal_width as usize) / col_length, 1);

    let mut lines = vec![vec![]];

    for dir in filtered_dirs {
        let path_string = dir.0;

        let last_line_index = lines.len() - 1;

        if lines[last_line_index].len() < cols_per_row {
            lines[last_line_index].push(path_string);
        } else {
            lines.push(vec![path_string]);
        }
    }

    print_lines(lines, col_length);
}

fn main() {
    let flags: Vec<String> = env::args().filter(|arg| arg.split("-").nth(0) == Some("")).collect();
    let path = env::args().filter(|arg| arg.split("-").nth(0) != Some("")).nth(1);

    // Get listing path. Defaults to "./"
    let listing_path = match path {
        None => String::from("./"),
        Some(arg_path) => arg_path.trim().to_string(),
    };

    let show_all = flags.iter().any(|flag| flag == &"-a");

    let long = flags.iter().any(|flag| flag == &"-l");

    // Get a list of paths from the listing path
    match fs::read_dir(listing_path) {
        Ok(listing_result) => display_list(listing_result, show_all, long),
        Err(err) => println!("Error: {}", err),
    }
}
