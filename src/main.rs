use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    // TODO: come back to double check the perf for large input
    let reader = BufReader::new(stdin.lock());

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        println!("Line {}: {}", i + 1, line);
    }
    Ok(())
}
