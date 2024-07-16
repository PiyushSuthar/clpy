use clap::arg;
use clipboard_win::{formats, get_clipboard, is_format_avail};
use std::{
    env,
    io::Write,
    path::{Path, PathBuf},
};

fn main() {
    let cli = clap::Command::new("clpy")
        .author("clpy contributors https://github.com/PiyushSuthar/clpy")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Save copied image from clipboard as an image file directly from your command line!")
        .arg(
            arg!(<file_name> "Name of the image file you want to save. If \"-\" data will be written to stdout")
                .required(false)
                .value_parser(clap::value_parser!(String))
        )
        .arg(arg!(--overwrite "Overwrite existing image file").required(false))
        .try_get_matches()
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        });

    if !is_format_avail(formats::CF_BITMAP) {
        eprintln!("No Image in clipboard");
        std::process::exit(1);
    }

    // Getting buffer frmo Clipboard
    let buffer_bitmap = get_clipboard(formats::Bitmap);

    // handling errors
    match buffer_bitmap {
        Ok(data) => {
            let file_path = cli.get_one::<String>("file_name").unwrap();
            if file_path == "-" {
                write_image_to_stdout(data)
            } else {
                let file_path = PathBuf::from(file_path);
                save_image_to_file(&file_path, data, cli.get_flag("overwrite"));
            }
        }
        Err(_) => {
            eprintln!("Please copy an Image first :)")
        }
    }
}

fn write_image_to_stdout(data: Vec<u8>) {
    std::io::stdout().write_all(&data).unwrap_or_else(|e| {
        eprintln!("Failed to write to stdout: {}", e);
        std::process::exit(1);
    });
}

fn save_image_to_file(file_path: &Path, content: Vec<u8>, overwrite: bool) {
    let image = image::load_from_memory(&content).unwrap_or_else(|_| {
        eprintln!("Failed to load image from clipboard");
        std::process::exit(1);
    });

    if file_path.exists() && !overwrite {
        eprintln!(
            "File {} already exists. Use --overwrite to overwrite it",
            file_path.display()
        );
        std::process::exit(1);
    }

    image.save(file_path).unwrap_or_else(|e| {
        eprintln!("Failed to save image to {}: {}", file_path.display(), e);
        std::process::exit(1);
    });
}
