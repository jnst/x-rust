mod hello_world;
mod introduction;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("The Rust Programming Language")
        .version("0.1")
        .author("jnst <jnst@outlook.com>")
        .about("Run sample code")
        .subcommand(SubCommand::with_name("hello")
            .about("Run hello_world.rs"))
        .subcommand(SubCommand::with_name("introduction")
            .about("Run introduction.rs")
            .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .takes_value(true)
                .help("Specify filename")))
        .get_matches();

    match matches.subcommand() {
        ("hello", Some(_matches)) => {
            hello_world::hello();
        }
        ("introduction", Some(_matches)) => {
            if _matches.is_present("file") {
                let filename = _matches.value_of("file").unwrap();
                introduction::print_or_create_file(filename);
            } else {
                let rect = introduction::Rectangle { width: 30, height: 25 };
                println!("{:#?}", rect);
                println!("The area of the rectangle is {} square pixels.", rect.area())
            }
        }
        _ => {
            unreachable!();
        },
    }
}
