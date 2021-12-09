fn read_file(path: String) -> Result<Vec<String>, String> {
    let pb = std::path::PathBuf::from(&path);
    if !pb.exists() {
        return Err(format!("Couldnt find file at path: {}", path));
    }
    let content = std::fs::read_to_string(pb).unwrap();
    Ok(content.lines().map(|e| e.to_string()).collect())
}

pub fn get_input(day: usize) -> Vec<String> {
    let path = format!("./resources/day{}.txt", day);
    let content = read_file(path);

    match content {
        Ok(content) => content,
        Err(_) => panic!("Couldnt find puzzle input for day {}", day),
    }
}