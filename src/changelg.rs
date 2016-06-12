extern crate git2;
extern crate regex;

use self::git2::{Repository, Commit};
use self::regex::Regex;

macro_rules! filter_option {
    ($e:expr) => (match $e { Ok(t) => t, Err(e) => return Some(Err(e)) })
}

macro_rules! option_match {
    ($e:expr) => (match $e { Ok(t) => t, Err(e) => panic!("err! - {}", e) })
}

// Todo: Return revwalk and defer iterating over comits to another function
pub fn get_commits(args: Vec<String>) {
    let repo = option_match!(Repository::open("./"));
    let mut revwalk = option_match!(repo.revwalk());

    option_match!(
        revwalk.push_range(
            &commit_range(&args[1][..], &args[2][..])[..]
        )
    );

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

fn commit_range(commit_from: &str, commit_to: &str) -> String {
    let mut commit_range = String::from("");
    
    commit_range.push_str(commit_from);
    commit_range.push_str("..");
    commit_range.push_str(commit_to);
    
    commit_range
}

fn print_to_stdout(commit: &Commit) {            
    for line in String::from_utf8_lossy(commit.message_bytes()).lines() {
        if match_changelog_identifier(line) {
            println!("{} by {}", strip_changelog_hashtag(line), commit.author());
        };
    };        
}

fn match_changelog_identifier(line: &str) -> bool {
    let re = Regex::new(r"^#changelog.*$").unwrap();
    re.is_match(line)
}

fn strip_changelog_hashtag(commit_msg: &str) -> &str {
    if commit_msg.to_lowercase().starts_with("#changelog ") {
        return &commit_msg[11..].trim_left();
    }
    commit_msg
}

#[test]
#[should_panic]
fn it_should_panic_on_error_in_option_match() {
    fn something_bad_happens<T>() -> Result<T, &'static str> {
        Err("Fail")    
    };

    match something_bad_happens() { 
        Ok(t) => t, 
        Err(e) => panic!("err! - {}", e) 
    }
}

#[test]
fn it_should_return_result_on_ok_in_option_match() {
    fn something_good_happens() -> Result<&'static str, &'static str> {
        Ok("Good")    
    };
    
    let result = match something_good_happens() { 
        Ok(t) => t, 
        Err(e) => panic!("err! - {}", e) 
    };

    assert_eq!(result, "Good");
}

#[test]
fn it_should_return_expected_commit_range_string(){
    let commit_range = commit_range("377d686351969f27f288dec2fb09d0d5431fcde1", "3763e0e3ff218cbdfbf99c68109a04d666e81abeto");
    assert_eq!(commit_range, "377d686351969f27f288dec2fb09d0d5431fcde1..3763e0e3ff218cbdfbf99c68109a04d666e81abeto");
}

#[test]
fn it_should_return_true_when_a_string_is_tagged_changelog_(){
    let result = match_changelog_identifier("#changelog Hello World");
    assert_eq!(result, true);
}

#[test]
fn it_should_return_false_when_a_string_is_not_tagged_changelog_(){
    let result = match_changelog_identifier("Hello World");
    assert_eq!(result, false);
}

#[test]
fn it_should_return_message_without_hashtag() {
    let result = strip_changelog_hashtag("#changelog This is a test commit message");
    assert_eq!(result, "This is a test commit message");
}

#[test]
fn it_should_return_message_without_hashtag_and_surplus_whitespace() {
    let result = strip_changelog_hashtag("#changelog     This is a test commit message");
    assert_eq!(result, "This is a test commit message");
}

#[test]
fn it_should_return_message_without_changes_if_not_changelog() {
    let result = strip_changelog_hashtag("This is a test commit message without a changelog hashtag");
    assert_eq!(result, "This is a test commit message without a changelog hashtag");
}