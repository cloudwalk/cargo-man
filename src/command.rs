extern crate clap;
use clap::{App, Arg};

pub struct Opts {
    pub path: String,
    pub table: String,
    pub field: Option<String>,
    pub set: Option<String>,
}

const DEFAULT_PATH: &str = "Cargo.toml";
const DEFAULT_TABLE: &str = "package";

pub fn start_command() -> Opts {
    let matches = App::new("Cargo man")
        .subcommand(
            App::new("man")
                .version("0.0.3")
                .author("CloudWalk <cloudwalk@cloudwalk.io>")
                .about("Manage cargo file information")
                .arg(
                    Arg::with_name("path")
                        .long("path")
                        .short("p")
                        .takes_value(true)
                        .help("The path of your cargo toml file")
                        .default_value(DEFAULT_PATH),
                )
                .arg(
                    Arg::with_name("table")
                        .long("table")
                        .short("t")
                        .takes_value(true)
                        .help("the table you want to select")
                        .default_value(DEFAULT_TABLE),
                )
                .arg(
                    Arg::with_name("field")
                        .long("field")
                        .short("f")
                        .takes_value(true)
                        .help("the field you want to see or set"),
                )
                .arg(
                    Arg::with_name("set")
                        .long("set")
                        .short("s")
                        .takes_value(true)
                        .requires("field")
                        .help("the new value of your field"),
                ),
        )
        .get_matches();

    if let Some(mtc) = matches.subcommand_matches("man") {
        return Opts {
            path: match mtc.value_of("path") {
                Some(t) => t.to_string(),
                None => DEFAULT_PATH.to_string(),
            },
            table: match mtc.value_of("table") {
                Some(t) => t.to_string(),
                None => DEFAULT_TABLE.to_string(),
            },
            field: match mtc.value_of("field") {
                Some(t) => Some(t.to_string()),
                None => None,
            },
            set: match mtc.value_of("set") {
                Some(t) => Some(t.to_string()),
                None => None,
            },
        };
    }

    Opts {
        path: DEFAULT_PATH.to_string(),
        table: DEFAULT_TABLE.to_string(),
        field: None,
        set: None,
    }
}
