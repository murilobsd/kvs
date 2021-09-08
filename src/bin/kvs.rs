use clap::{App, Arg, SubCommand};

static AUTHORS: &str = env!("CARGO_PKG_VERSION");
static NAME: &str = env!("CARGO_PKG_NAME");
static VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about("command-line key-value store")
        .subcommands(vec![
            SubCommand::with_name("get")
                .arg(Arg::with_name("key").required(true).index(1)),
            SubCommand::with_name("set")
                .arg(Arg::with_name("key").required(true).index(1))
                .arg(Arg::with_name("value").required(true).index(2)),
            SubCommand::with_name("rm")
                .arg(Arg::with_name("key").required(true).index(1)),
        ])
        .get_matches();

    match matches.subcommand_name() {
        Some("set") => unimplemented!("unimplemented"),
        Some("get") => unimplemented!("unimplemented"),
        Some("rm") => unimplemented!("unimplemented"),
        _ => unimplemented!("unimplemented"),
    };
}
