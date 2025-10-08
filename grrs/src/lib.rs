use std::fs::File;
use std::io::{BufReader, prelude::*};

pub fn find_matches(
    reader: BufReader<File>,
    pattern: &str,
    mut writer: impl std::io::Write,
) -> std::io::Result<()> {
    for line in reader.lines() {
        // Ignore errors on lines
        let line = line?;
        // Check if the line contains the pattern
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use assert_fs::prelude::FileWriteStr;

    use super::*;

    #[test]
    fn find_a_match() {
        let mut result = Vec::new();

        // Create a temporary file for testing
        use assert_fs::NamedTempFile;

        let temp_file = NamedTempFile::new("temp.txt").unwrap();
        temp_file
            .write_str("This is a test\nfinder\nnot this\n")
            .unwrap();

        let file = File::open(temp_file.path()).unwrap();
        let reader = BufReader::new(file);

        find_matches(reader, "finder", &mut result).unwrap();

        let result_string = String::from_utf8(result).unwrap();
        assert_eq!(result_string, "finder\n");
    }
}
