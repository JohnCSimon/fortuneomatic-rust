use std::fs;
use std::fs::DirEntry;
use std::io;
use std::path::Path;
pub async fn readDirectory(path: &str) {
    let y = std::path::Path::new(path);

    // for entry in fs::read_dir(dir)? {
    //     let entry = entry?;
    //     let path = entry.path();
    //     if path.is_dir() {
    //         visit_dirs(&path, cb)?;
    //     } else {
    //         cb(&entry);
    //     }
    // }

    println!("Im in single module");
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
