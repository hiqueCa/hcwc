use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command_option: &String = &args[1];
    let file_name: &String = &args[2];

    let file_content: String = fs::read_to_string(&file_name).expect(
        "Could not ready specified file content!"
    );

    if command_option == "-c" {
        let file_bytes: &usize = &file_content.len();
        println!("{file_bytes}");
    }
}
