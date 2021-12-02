use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn path_to_lines<P>(path: P) -> Result<Lines<BufReader<File>>, io::Error>
where
    P: AsRef<Path>,
{
    Ok(BufReader::new(File::open(path)?).lines())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut horizontal: isize = 0;
    let mut depth: isize = 0;
    let mut aim: isize = 0;
    
    for line in path_to_lines("input")? {
        if let Ok(line) = line {
            let tokens = line.split(' ').collect::<Vec<&str>>();
            assert_eq!(tokens.len(), 2);
            match (tokens[0], tokens[1].parse::<isize>()) {
                ("forward", Ok(n)) => {
                    horizontal += n;
                    depth += aim * n;
                    if depth < 0 {
                        depth = 0;
                    }
                }
                ("down", Ok(n)) => aim += n,
                ("up", Ok(n)) => aim -= n,
                _ => panic!("Invalid Input"),
            }
        }
    }
    println!(
        "horizontal = {} depth = {}\n{}",
        horizontal,
        depth,
        horizontal * depth
    );
    Ok(())
}
