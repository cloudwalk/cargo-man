extern crate clap;
use crate::version::get_version;
use clap::{App, Arg};

pub struct Opts {
    pub path: String,
    pub table: String,
    pub field: Option<String>,
    pub set: Option<String>,
    pub bump: Option<Bump>,
}

pub struct Bump {
    pub kind: String,
    pub quiet: bool,
}

const DEFAULT_PATH: &str = "Cargo.toml";
const DEFAULT_TABLE: &str = "package";

pub fn start_command() -> Opts {
    let matches = App::new("Cargo man")
        .subcommand(
            App::new("man")
                .version(get_version().as_str())
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
                )
                .subcommand(
                    App::new("bump")
                        .version(get_version().as_str())
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
                            Arg::with_name("type")
                                .long("type")
                                .short("t")
                                .takes_value(true)
                                .help("the type of bump you want (patch, minor, major)"),
                        )
                        .arg(
                            Arg::with_name("quiet")
                                .long("quiet")
                                .short("q")
                                .takes_value(false)
                                .help(
                                    "if this flag is enabled it will not prompt any confirmation",
                                ),
                        ),
                ),
        )
        .get_matches();

    if let Some(mtc) = matches.subcommand_matches("man") {
        let mut bump_path = String::new();
        let bump = if let Some(bump_r) = mtc.subcommand_matches("bump") {
            bump_path = match bump_r.value_of("path") {
                Some(t) => t.to_string(),
                None => DEFAULT_PATH.to_string(),
            };
            Some(Bump {
                kind: match bump_r.value_of("type") {
                    Some(t) => t.to_string(),
                    None => "patch".to_string(),
                },
                quiet: bump_r.is_present("quiet"),
            })
        } else {
            None
        };

        return Opts {
            path: if bump_path.is_empty() {
                match mtc.value_of("path") {
                    Some(t) => t.to_string(),
                    None => DEFAULT_PATH.to_string(),
                }
            } else {
                bump_path
            },
            table: match mtc.value_of("table") {
                Some(t) => t.to_string(),
                None => DEFAULT_TABLE.to_string(),
            },
            field: mtc.value_of("field").map(|t| t.to_string()),
            set: mtc.value_of("set").map(|t| t.to_string()),
            bump,
        };
    }

    Opts {
        path: DEFAULT_PATH.to_string(),
        table: DEFAULT_TABLE.to_string(),
        field: None,
        set: None,
        bump: None,
    }
}
