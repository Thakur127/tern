use clap::{Parser, Subcommand};
use tern::commands;
use tern::constants::{DEFAULT_MIGRATIONS_DIR, TERN_DATABASE_URL};

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Initialize the migrations folder")]
    Init {
        #[arg(
            short,
            long,
            default_value = "false",
            help = "Force init, rewrites the config file and deletes all migrations history, becareful"
        )]
        force: bool,

        #[arg(
            short,
            long,
            default_value= DEFAULT_MIGRATIONS_DIR,
            help = "Path to the migrations folder"
        )]
        path: Option<String>,

        #[arg(
            short,
            long,
            help = format!("Database url. Either set this or set the {TERN_DATABASE_URL} env var")
        )]
        db_url: Option<String>,
        // #[arg(short, long, default_value = DEFAULT_SCHEMA_NAME , help = "Database table name for recording the migrations history")]
        // schema: Option<String>,
    },

    #[command(about = "Generate a new empty migration folder with up.sql and down.sql files")]
    Generate {
        #[arg(help = "Migration name`")]
        name: String,
    },

    #[command(
        about = "Apply the migrations, default is to apply all the migrations, use --name to apply a specific migration"
    )]
    Upgrade {
        #[arg(short, long, help = "Apply a specific migration")]
        name: Option<String>,
    },

    #[command(
        about = "Rollback the migrations, default is to rollback all the migrations, use --steps to rollback a specific number of migrations"
    )]
    Rollback {
        #[arg(
            short,
            long,
            default_value = "1",
            help = "Rollback a specific number of migrations"
        )]
        steps: u32,

        #[arg(
            short,
            long,
            default_value = "false",
            help = "Rollback all the migrations. It will be preferred over --steps"
        )]
        all: bool,

        #[arg(
            short,
            long,
            help = "Rollback a specific migration. It has higher priority than --steps and --all"
        )]
        name: Option<String>,
    },

    #[command(about = "List the migrations")]
    List {
        #[arg(short, long, default_value = "false", help = "List applied migrations")]
        applied: bool,
    },

    #[command(
        about = "Check the status of the migrations. Checks whether migrations are applied or staged(yet to apply). Can be alternative to `list` command"
    )]
    Status,

    #[command(
        about = "Reset the migrations. CAUTION: Rollbacks all the migrations and re applies all of them. Use only when you know what are you doing."
    )]
    Reset {
        #[arg(short, long, default_value = "false", help = "Force reset")]
        force: bool,
    },
}

#[derive(Parser)]
#[command(name = "tern")]
#[command(about = "A simple db migration tool written in rust", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init(); // export RUST_LOG=trace

    let cli = Cli::parse();

    match cli.command {
        Commands::Init {
            force,
            path,

            db_url,
        } => Ok(commands::init::handle(path, force, db_url)?),
        Commands::Generate { name } => Ok(commands::generate::handle(name)?),
        Commands::Upgrade { name } => Ok(commands::upgrade::handle(name)?),
        Commands::Rollback { steps, name, all } => {
            Ok(commands::rollback::handle(steps, name, all)?)
        }
        Commands::List { applied } => Ok(commands::list::handle(applied)?),
        Commands::Status => Ok(commands::status::handle()?),
        Commands::Reset { force } => Ok(commands::reset::handle(force)?),
    }
}
