use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "logos")]
#[command(about = "LOGOS CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the database
    Init,
    /// Run a node
    Run {
        node_id: String,
    },
    /// Visualize the graph
    Viz,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            println!("Initializing database...");
        }
        Commands::Run { node_id } => {
            println!("Running node: {}", node_id);
        }
        Commands::Viz => {
            println!("Visualizing graph...");
        }
    }
}
