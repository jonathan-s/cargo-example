#![allow(dead_code)]

extern crate clap;
extern crate cargo_example;
use clap::{
    App,
    SubCommand,
    Arg,
};

use cargo_example::run_example;

fn main() {
    let matches = App::new("cargo example")
        .version("v1.0-beta")
        .bin_name("cargo")
        .about("Run examples from from a github repo on your command line!")
        .author("Jonathan Sundqvist")
        .subcommand(SubCommand::with_name("example")
            .arg(Arg::with_name("crate")
                .help("The name of your crate. Will give you a list of all example files.")
                .index(1)
            )
            .arg(Arg::with_name("example_rs")
                .help("The name of the example file.")
                .requires("crate")
                .index(2)
            )
        )
        .get_matches();

    let command = &matches.subcommand;
    dbg!(matches.subcommand_matches("example"));
    // println!("{:?}", &command);
    match matches.subcommand_name() {
        Some("example") => run_example(matches.subcommand_matches("example").unwrap()),
        _ => ()
    }

    // if command.is_some() {
    //     let command = &command.as_ref().unwrap();

        // let _result = handle_command(command);
}
