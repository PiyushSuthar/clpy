use clipboard_win::{formats, get_clipboard, is_format_avail};
use std::{env, fs};

use crate::utils::{path_exists, random_string};

mod utils;

fn main() {
    // Checking if there's an image in clipboard
    if is_format_avail(formats::CF_BITMAP) {
        handler()
    } else {
        println!("No Image in clipboard");
        println!("Please copy an Image first :)");
    }
}

fn handler() {
    // Taking args in cli
    let args: Vec<String> = env::args().collect();

    // Getting buffer frmo Clipboard
    let buffer_bitmap = get_clipboard(formats::Bitmap);

    // handling errors
    match buffer_bitmap {
        Ok(data) => {
            // If name is parsed, use that name for image.
            if args.len() > 1 {
                // Taking name from args
                let file_name = &args[1];
                save_image(file_name.as_str(), data)
            } else {
                // Else use clpy_image as the name
                save_image("clpy_image", data)
            }
        }
        Err(_) => {
            // Incase of error
            println!("Please copy an Image first :)")
        }
    }
}

fn save_image(file_name: &str, content: Vec<u8>) {
    let file_path = format!("{}.png", file_name);

    if path_exists(&file_path) {
        // Getting random string
        let random = random_string();
        println!(
            "{file_name}.png already exists. So we saved the image as {file_name}-{random}.png ",
            file_name = file_name,
            random = random
        );
        // Writing image
        fs::write(format!("{}-{}.png", file_name, random), content).expect("Failed to create Image")
    } else {
        // Logging out a message
        println!("Saving copied image as {}", file_path);
        // Writing image
        fs::write(file_path, content).expect("Failed to create Image")
    }
}
