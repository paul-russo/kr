use std::fs;
use std::env::args;
use std::os::unix::fs::PermissionsExt;

fn display_list(path_list: fs::ReadDir) {
    for path in path_list {
        let the_path = path.unwrap();
        let metadata = the_path.metadata().unwrap();

        // {:o} formats the permissions number in octal
        println!("{} - {:o}", the_path.path().display(), metadata.permissions().mode());
    }
}

fn main() {
    // Get listing path. Defaults to "./"
    let listing_path = match args().nth(1) {
        None => String::from("./"),
        Some(arg_path) => arg_path.trim().to_string(),
    };

    // Get a list of paths from the listing path
    match fs::read_dir(listing_path) {
        Ok(listing_result) => display_list(listing_result),
        Err(err) => println!("Error: {}", err),
    }
}
