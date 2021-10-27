use std::fs;
use std::fs::DirEntry;
use std::fs::File;
use std::io;
use std::io::{prelude::*, BufReader};
use std::iter::FromIterator;
use std::path::Path;
use std::vec;

pub fn parse_fortune_file(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
    // let xxy: Vec<&str> = s.split(",").collect();
    // Ok(xxy)
}

// pub fn parse_fortune_filezzzz(s: &str) -> io::Result<Vec<String>> {
//     let mut file = File::open(s)?;
//     let mut s = String::new();
//     file.read_to_string(&mut s)?;

//     let xxy: Vec<&String> = s.split(",").collect();

//     Ok(s)
//     // let xxy: Vec<&str> = s.split(",").collect();
//     // Ok(xxy)
// }

fn words_by_line(s: &str) -> std::vec::Vec<std::string::String> {
    let y = parse_fortune_file(s).unwrap();
    Vec::from_iter(y.split("\n%\n").map(String::from))
}

// fn dofilter(xyz &std::result::Result<std::fs::DirEntry, std::io::Error>){

// }

pub fn visit_dirs(dir: &Path) -> std::vec::Vec<std::path::PathBuf> {
    let mut y = Vec::new();
    if dir.is_dir() {
        // let z = fs::read_dir(dir)?
        //     .filter (|x| !x.as_ref().unwrap().path().is_dir())
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                // visit_dirs(&path, cb)?;
            } else {
                println!("{:?}", path);
                y.push(path);
            }
        }
    }
    y
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_fortune_file_test() {
        let result = words_by_line("fortunes/animals");
        let mut count = 0;
        for i in result {
            count += 1;
            println!("line");
            println!("{:?} {:?}", count, i)
        }
    }

    #[test]
    fn parse_fortune_test() {
        let x = Path::new("fortunes");
        visit_dirs(x);
    }
}
