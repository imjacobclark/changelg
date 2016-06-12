mod changelg;

use std::env;

fn main() {
    changelg::get_commits(env::args().collect());
}