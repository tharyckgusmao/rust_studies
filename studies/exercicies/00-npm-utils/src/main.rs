use ::clap::{Args, Parser, Subcommand};

mod api;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "npm-utils - small utils about publish and not publish packages")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Pack(Pack),
}

#[derive(Args)]
struct Pack {
    module: Option<String>,
    #[arg(short, long)]
    find: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Pack(command)) => match command.module {
            Some(ref _name) => {
                let pack = api::commands::pack(_name);
                println!("{}", pack);
            }
            None => match command.find {
                Some(ref find) => {
                    api::commands::choose(Some(find));
                }
                None => {
                    api::commands::choose(None);
                }
            },
        },
        None => {}
    }
}
