extern crate git2;

use git2::Repository;

fn main() {
    let repo = match Repository::open("./") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };
    
    let mut revwalk = match repo.revwalk() {
        Ok(revwalk) => revwalk,
        Err(_) => panic!("Failed to revwalk")
    };
    
    println!("{}", revwalk.count());
}
