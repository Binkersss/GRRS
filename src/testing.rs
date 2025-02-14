use crate::find_and_print_matches;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Write};
    use std::fs::File;
    use tempfile::NamedTempFile; // Use the `tempfile` crate for temporary files

    #[test]
    fn test_find_and_print_matches() {
        // Create a temporary file and write test input to it
        let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
        writeln!(temp_file, "hello world").expect("Failed to write to temporary file");
        writeln!(temp_file, "goodbye world").expect("Failed to write to temporary file");
        writeln!(temp_file, "rust is fun").expect("Failed to write to temporary file");
        writeln!(temp_file, "world of rust").expect("Failed to write to temporary file");

        // Reopen the file for reading
        let file = File::open(temp_file.path()).expect("Failed to open temporary file");
        let mut reader = BufReader::new(file);

        // Create a mock writer to capture the output
        let mut output = Vec::new();

        // Call the function with a pattern
        let mut line = String::new();
        find_and_print_matches(&mut line, "world", &mut reader, &mut output);

        // Convert the output to a string for easier comparison
        let output_str = String::from_utf8(output).expect("Invalid UTF-8 in output");

        // Define the expected output
        let expected_output = "hello world\n\ngoodbye world\n\nworld of rust\n\n";

        // Assert that the output matches the expected result
        assert_eq!(output_str, expected_output);
    }

    #[test]
    fn test_find_and_print_matches_no_matches() {
        // Create a temporary file and write test input to it
        let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
        writeln!(temp_file, "hello world").expect("Failed to write to temporary file");
        writeln!(temp_file, "goodbye world").expect("Failed to write to temporary file");
        writeln!(temp_file, "rust is fun").expect("Failed to write to temporary file");
        writeln!(temp_file, "world of rust").expect("Failed to write to temporary file");

        // Reopen the file for reading
        let file = File::open(temp_file.path()).expect("Failed to open temporary file");
        let mut reader = BufReader::new(file);

        // Create a mock writer to capture the output
        let mut output = Vec::new();

        // Call the function with a pattern that doesn't exist
        let mut line = String::new();
        find_and_print_matches(&mut line, "python", &mut reader, &mut output);

        // Convert the output to a string for easier comparison
        let output_str = String::from_utf8(output).expect("Invalid UTF-8 in output");

        // Define the expected output (empty because no matches)
        let expected_output = "";

        // Assert that the output matches the expected result
        assert_eq!(output_str, expected_output);
    }

    #[test]
    fn test_find_and_print_matches_empty_file() {
        // Create an empty temporary file
        let temp_file = NamedTempFile::new().expect("Failed to create temporary file");

        // Reopen the file for reading
        let file = File::open(temp_file.path()).expect("Failed to open temporary file");
        let mut reader = BufReader::new(file);

        // Create a mock writer to capture the output
        let mut output = Vec::new();

        // Call the function with a pattern
        let mut line = String::new();
        find_and_print_matches(&mut line, "world", &mut reader, &mut output);

        // Convert the output to a string for easier comparison
        let output_str = String::from_utf8(output).expect("Invalid UTF-8 in output");

        // Define the expected output (empty because the file is empty)
        let expected_output = "";

        // Assert that the output matches the expected result
        assert_eq!(output_str, expected_output);
    }
}