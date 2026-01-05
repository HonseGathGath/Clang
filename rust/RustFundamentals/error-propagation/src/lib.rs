use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error},
};

pub fn sum_integers_from_file(file_path: &str) -> Result<i32, io::Error> {
    let f = File::open(file_path)?;
    let lines = BufReader::new(f);

    let mut sum: i32 = 0;

    for line in lines.lines() {
        let line = line?;

        let parsed_line: i32 = line
            .parse()
            .map_err(|_| Error::new(io::ErrorKind::InvalidData, "failed to aprse"))?;

        sum += parsed_line;
    }

    Ok(sum)
}

// Example usage
pub fn main() {
    let file_path = "numbers.txt";

    match sum_integers_from_file(file_path) {
        Ok(sum) => println!("The sum is: {}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}
