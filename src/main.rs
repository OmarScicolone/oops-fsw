mod packet; 

fn main() {
    let bytes = load_files("data/fsw-output.txt");
    println!("Byte totali: {}", bytes.len());
}

fn load_files(path_file: &str) -> Vec<u8> {
    let content = std::fs::read_to_string(path_file).expect("file non trovato");
    content
        .split_whitespace()
        .map(|token| u8::from_str_radix(token, 16).expect("token non valido"))
        .collect()
}
