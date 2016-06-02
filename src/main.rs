extern crate git2;
extern crate regex;

use std::env;
use git2::Repository;
use regex::Regex;

macro_rules! filter_try {
    ($e:expr) => (match $e { Ok(t) => t, Err(e) => return Some(Err(e)) })
}

macro_rules! option_match {
    ($e:expr) => (match $e { Ok(t) => t, Err(e) => panic!("err! - {}", e) })
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let repo = option_match!(Repository::open("./"));
    let mut revwalk = option_match!(repo.revwalk());
    let commit_from: &str = &args[1][..];
    let commit_to: &str = &args[2][..];
    let mut commit_range = String::from("");
    
    commit_range.push_str(commit_from);
    commit_range.push_str("..");
    commit_range.push_str(commit_to);
    
    option_match!(revwalk.push_range(&commit_range[..]));

    let revwalk = revwalk.filter_map(|id| {
        let id = option_match!(id);
        let commit = filter_try!(repo.find_commit(id));        
        Some(Ok(commit))
    });
    
    for commit in revwalk {
        let commit = option_match!(commit);
                
        for line in String::from_utf8_lossy(commit.message_bytes()).lines() {
            let re = Regex::new(r"^#changelog.*$").unwrap();
            
            if re.is_match(line) {
                    println!("{} by {}", line, commit.author());
            }
        }        
    };
}
