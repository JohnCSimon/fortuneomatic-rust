use std::path::Path;
mod readfortunefile;

fn main() {
    let x = Path::new("fortuness");
    readfortunefile::visit_dirs(x);
}
