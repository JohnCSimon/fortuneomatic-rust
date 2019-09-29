mod readfortunefile;

use postgres::{Connection, TlsMode};
use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

fn cowsay_string() -> String {
    let fortune_result = Command::new("sh")
        .arg("-c")
        .arg("fortune | cowsay")
        .output()
        .expect("failed to execute process")
        .stdout;
    String::from_utf8(fortune_result).unwrap()
}

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() {
    let pathStr = "/home/johnsimon/git/rust/fortuneomatic-rust";

    let path = Path::new(pathStr);
    let result = readfortunefile::visit_dirs(path);

    let xx = result.first().unwrap();
    println!("{:?}", xx);
    // result.push();
    // let xx: String = cowsay_string();

    // let conn = Connection::connect("postgresql://postgres@localhost:5432", TlsMode::None).unwrap();

    // conn.execute(
    //     "CREATE TABLE person (
    //                 id              SERIAL PRIMARY KEY,
    //                 name            VARCHAR NOT NULL,
    //                 data            BYTEA
    //               )",
    //     &[],
    // )
    // .unwrap();

    // let me = Person {
    //     id: 0,
    //     name: "Steven".to_owned(),
    //     data: None,
    // };
    // conn.execute(
    //     "INSERT INTO person (name, data) VALUES ($1, $2)",
    //     &[&me.name, &me.data],
    // )
    // .unwrap();

    // for row in &conn
    //     .query("SELECT id, name, data FROM person", &[])
    //     .unwrap()
    // {
    //     let person = Person {
    //         id: row.get(0),
    //         name: row.get(1),
    //         data: row.get(2),
    //     };
    //     println!("Found person {}", person.name);
    // }

    // println!("{:?}", xx);
    // println!("---- !---- ")
}
