use std::fs::File;
use std::io::{self, BufRead, *};
use std::path::Path;

fn main() {
    let path = Path::new("Cargo.toml");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(e) => panic!("couldn't read {}: {}", display, e),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(e) => panic!("couldn't read {}: {}", display, e),
        Ok(_) => println!("{} contains:\n{}", display, s),
    }

    if let Ok(lines) = read_lines("./Cargo.toml") {
        for (i, line) in lines.enumerate() {
            if let Ok(s) = line {
                println!("{:3}: {}", i + 1, s);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
