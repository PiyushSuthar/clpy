use clipboard_win::{formats, get_clipboard};
use std::{env, fs};

fn main() {
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
                handle_image_data(file_name.as_str(), data)
            } else {
                // Else use clpy_image as the name
                handle_image_data("clpy_image", data)
            }
        }
        Err(_) => {
            // Incase of error
            println!("Please copy an Image first :)")
        }
    }
}

fn handle_image_data(file_name: &str, content: Vec<u8>) {
    // Logging out a message
    println!("Saving copied image as {}.png", file_name);

    // Writing image
    fs::write(format!("{}.png", file_name), content).expect("Failed to create Image")
}
