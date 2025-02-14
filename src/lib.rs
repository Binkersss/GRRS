use std::io::BufRead;

pub fn find_and_print_matches(line: &mut String, pattern: &str,
                              reader: &mut std::io::BufReader<std::fs::File>,
                              mut writer: impl std::io::Write) {
    loop {
        line.clear();

        let bytes_read = reader.read_line(line).expect("could not read line");

        if bytes_read == 0 {
            break;
        }

        if line.contains(pattern) {
            writeln!(writer, "{}", line).expect("");
        }
    }
}