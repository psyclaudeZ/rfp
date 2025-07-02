use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let lines = parse_lines();

    for (i, line) in lines?.iter().enumerate() {
        let line = line;
        println!("Line {}: {}", i + 1, line);
    }
    Ok(())
}

fn parse_lines() -> io::Result<Vec<String>> {
    let stdin = io::stdin();
    // TODO: come back to double check the perf for large input
    let reader = BufReader::new(stdin.lock());
    return reader.lines().collect();
}
