use std::fs;

pub fn read_today_data_file<'a>(day: String) -> String {
    let path = format!("./problems/{day}_data.txt");
    let file: String = read_file(&path);
    file
}

fn read_file(path: &String) -> String {
    // Returns all contents of a file
    let contents: String = fs::read_to_string(path).expect("Failed to read file {path}");
    return contents;
}
