use walkdir::WalkDir;

pub fn search(query: &String) -> Vec<String> {
    WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
        .map(|e| e.path().to_string_lossy().into_owned())
        .filter(|s| s.contains(query.as_str()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let query = String::from("src");
        let results = search(&query);
        assert!(results.iter().any(|p| p.contains("src")));
    }
}
