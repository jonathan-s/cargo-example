extern crate clap;
extern crate dirs;
extern crate reqwest;
// use std::path::Path;

use reqwest::Client;
use std::io::Error;
use std::io::ErrorKind::AlreadyExists;
use std::fs::create_dir;
use std::collections::HashMap;
use clap::SubCommand;


fn create_directory() -> Result<&'static str, Error> {
    let path = dirs::home_dir().unwrap();
    let home = path.join(".cargo/examples");
    let example_dir = home;
    let result = create_dir(example_dir);
    match result {
        Ok(_result) => Ok(""),
        Err(ref e) if e.kind() == AlreadyExists => Ok(""),
        Err(e) => Err(e)
    }
}

fn request_crate(crate_: &str) -> &str{
    const BASE_URL: &'static str = "https://crates.io/api/v1/crates/";
    let url = format!("{}{}", BASE_URL, crate_);
    let client = Client::new();
    let resp = client.get(&url[..]);
    dbg!(resp);

    ""

}

fn execute_example(crate_: &str, example: &str) {

}

fn list_examples(crate_: &str) {

}

pub fn run_example(matches: &clap::ArgMatches) {
    // create_directory()?; need to return an error first. fix this later
    let crate_ = matches.value_of("crate").unwrap();
    let example = matches.value_of("example_rs");
    if example.is_some() {
        execute_example(crate_, example.unwrap());
    }
    else {
        list_examples(crate_);
    }
}


// api for crates.io
// get the crate name.


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_crate() {
        request_crate("rand");
    }
}
