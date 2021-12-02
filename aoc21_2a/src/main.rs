use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};
use std::path::Path;

fn path_to_lines<P>(path: P) -> Result<Lines<BufReader<File>>, Error>
where
    P: AsRef<Path>,
{
    Ok(BufReader::new(File::open(path)?).lines())
}

fn main() -> Result<(), Error> {
    let mut horizontal = 0;
    let mut depth = 0;
    for line in path_to_lines("input")? {
        if let Ok(line) = line {
            let tokens = line.split(' ').collect::<Vec<&str>>();
            assert_eq!(tokens.len(), 2);
            match (tokens[0], tokens[1].parse::<isize>()) {
                ("forward", Ok(n)) => horizontal += n,
                ("down", Ok(n)) => depth += n,
                ("up", Ok(n)) => {
                    depth -= n;
                    if depth < 0 {
                        depth = 0;
                    };
                }
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
