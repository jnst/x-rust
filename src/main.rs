mod hello_world;
mod introduction;
use clap::{App, SubCommand};

fn main() {
    let matches = App::new("The Rust Programming Language")
        .version("0.1")
        .author("jnst <jnst@outlook.com>")
        .about("Run sample code")
        .subcommand(SubCommand::with_name("hello"))
            .about("Run hello_world.rs")
        .subcommand(SubCommand::with_name("introduction"))
            .about("Run introduction.rs")
        .get_matches();

    match matches.subcommand() {
        ("hello", Some(_matches)) => {
            hello_world::hello();
        }
        ("introduction", Some(_matches)) => {
            let rect = introduction::Rectangle { width: 30, height: 25 };
            println!("{:#?}", rect);
            println!("The area of the rectangle is {} square pixels.", rect.area())
        }
        _ => {
            unreachable!();
        },
    }
}
