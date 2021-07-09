mod hello_world;
use clap::{App, SubCommand};

fn main() {
    let matches = App::new("Sample code")
        .version("0.1")
        .author("jnst <jnst@outlook.com>")
        .about("Run sample code")
        .subcommand(SubCommand::with_name("hello"))
            .about("Run hello world")
        .get_matches();

    match matches.subcommand() {
        ("hello", Some(_matches)) => {
            crate::hello_world::hello();
        }
        _ => unreachable!(),
    }
}
