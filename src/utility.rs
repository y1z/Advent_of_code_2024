use std::path::Path;

pub fn read_file(file_path: &str) -> String {
    let path = Path::new(file_path);

    let path_as_string = path.to_str().unwrap();
    print!("{}\n", path_as_string);

    if let Ok(result) = std::fs::read_to_string(path) {
        print!("Read file : '{}'\n\n", file_path);
        return result;
    }

    print!("ERROR : COULD NOT READ FILE\n\n");
    return String::new();
}
