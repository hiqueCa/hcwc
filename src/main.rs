use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name: &String = &args[2];

    let file_content: String = fs::read_to_string(&file_name).expect(
        "Could not ready specified file content!"
    );
    dbg!(file_content);
}
