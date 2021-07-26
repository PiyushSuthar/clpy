use std::path;

use random_string::generate;

// Function to check if file exists
pub fn path_exists(file_path: &String) -> bool {
    path::Path::new(file_path).exists()
}

pub fn random_string() -> String {
    let charset = "123456abcdefgh";
    generate(4, charset)
}
