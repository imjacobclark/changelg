extern crate git2;
extern crate regex;

#[cfg(not(test))] use std::env;

use git2::{Repository, Commit};
use regex::Regex;

macro_rules! filter_option {
    ($e:expr) => (match $e { Ok(t) => t, Err(e) => return Some(Err(e)) })
}

macro_rules! option_match {
    ($e:expr) => (match $e { Ok(t) => t, Err(e) => panic!("err! - {}", e) })
}

fn main() {
    let repo = option_match!(Repository::open("./"));
    let mut revwalk = option_match!(repo.revwalk());

    option_match!(revwalk.push_range(&commit_range()[..]));

    let revwalk = revwalk.filter_map(|id| {
        let id = option_match!(id);
        let commit = filter_option!(repo.find_commit(id));        
        Some(Ok(commit))
    });
    
    for commit in revwalk {
        let commit = option_match!(commit);
        print_to_stdout(&commit);
    }
}

fn print_to_stdout(commit: &Commit) {            
    for line in String::from_utf8_lossy(commit.message_bytes()).lines() {
        let re = Regex::new(r"^#changelog.*$").unwrap();
        
        if re.is_match(line) {
            println!("{} by {}", line, commit.author());
        }
    };        
}

fn commit_range() -> String {
    let args: Vec<_> = env::args().collect();
    let commit_from: &str = &args[1][..];
    let commit_to: &str = &args[2][..];
    let mut commit_range = String::from("");
    
    commit_range.push_str(commit_from);
    commit_range.push_str("..");
    commit_range.push_str(commit_to);
    
    commit_range
}

#[cfg(test)] mod env {
    use std::vec;
    pub fn args() -> vec::IntoIter<&'static str> { vec!["changelg", "377d686351969f27f288dec2fb09d0d5431fcde1", "3763e0e3ff218cbdfbf99c68109a04d666e81abeto"].into_iter() }
}

#[test]
#[should_panic]
fn it_should_panic_on_error_in_option_match() {
    fn something_bad_happens<T>() -> Result<T, &'static str> {
        Err("Fail")    
    };
    
    option_match!(something_bad_happens());
}

#[test]
fn it_should_return_result_on_ok_in_option_match() {
    fn something_good_happens() -> Result<&'static str, &'static str> {
        Ok("Good")    
    };
    
    let result = option_match!(something_good_happens());
    assert_eq!(result, "Good");
}

#[test]
fn it_should_return_expected_commit_range_string(){
    assert_eq!(commit_range(), "377d686351969f27f288dec2fb09d0d5431fcde1..3763e0e3ff218cbdfbf99c68109a04d666e81abeto");
}