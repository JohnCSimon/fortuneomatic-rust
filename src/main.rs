use std::path::Path;

use fortuneomatic::visit_dirs;

fn main() {
    let x = Path::new("fortuness");
    visit_dirs(x);
}
