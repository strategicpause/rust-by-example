use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version)]

struct Cli {
    // The command macro is used to define subcommands for the CLI. In this case we have three
    // commands: set, get, and rm.
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Set(SetArgs),
    Get(GetArgs),
    Rm(RmArgs),
}

#[derive(Args)]
struct SetArgs {
    #[arg(required = true)]
    key: String,
    #[arg(required = true)]
    value: String,
}

#[derive(Args)]
struct GetArgs {
    #[arg(required = true)]
    key: String,
}

#[derive(Args)]
struct RmArgs {
    #[arg(required = true)]
    key: String,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Get(_args)) => {
            panic!("unimplemented");
        }
        Some(Commands::Set(_args)) => {
            panic!("unimplemented");
        }
        Some(Commands::Rm(_args)) => {
            panic!("unimplemented");
        }
        None => {
            panic!("no args");
        }
    }
}
