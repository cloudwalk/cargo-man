use clap::{AppSettings, Clap};

#[derive(Clap)]
pub struct Root {
    #[clap(subcommand)]
    pub sub_comands: SubCommands,
}

#[derive(Clap)]
pub enum SubCommands {
    #[clap(version = "0.1.0", author = "CloudWalk <cloudwalk@cloudwalk.io>")]
    #[clap(setting = AppSettings::ColoredHelp)]
    Man(Opts),
}

#[derive(Clap)]
pub struct Opts {
    #[clap(short, long, default_value = "Cargo.toml")]
    pub path: String,

    #[clap(short, long)]
    pub field: Option<String>,

    #[clap(short, long)]
    pub set: Option<String>,
}

pub fn start_command() -> Opts {
    let r = Root::parse();
    match r.sub_comands {
        SubCommands::Man(t) => t,
    }
}
