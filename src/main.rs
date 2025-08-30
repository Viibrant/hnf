use clap::{Parser, Subcommand};
use hnf::Client;

#[derive(Parser)]
#[command()]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Top {
        #[arg(short, long, default_value_t = 10)]
        number: usize,
    },
    New {
        #[arg(short, long, default_value_t = 10)]
        number: usize,
    },
    Story {
        #[arg(short, long, default_value_t = 10)]
        number: usize,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let client = Client::new();

    match args.command {
        Commands::Top { number } => {
            print!("Fetching top ids");
            let ids = client.fetch_top_ids(number)?;
            let count = std::cmp::min(ids.len(), number as usize);
            println!("{:#?}", &ids[..count]);
        }
        Commands::New { number } => {
            println!("Fetching top stories");
            let ids = client.fetch_new_stories(number)?;
            let count = std::cmp::min(ids.len(), number as usize);
            println!("{:#?}", &ids[..count]);
        }
        Commands::Story { number } => {
            println!("Fetching story");
            let story = client.fetch_story(number)?;
            println!("{:#?}", &story);
        }
    }

    Ok(())
}
