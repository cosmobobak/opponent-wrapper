#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod cliargs;
mod lichess;
mod engine;
mod user;
mod gametype;

pub const LICHESS_TOKEN: &str = include_str!("../token.txt");
pub const LICHESS_HOST: &str = "https://lichess.org";

pub const VIRIDITHAS_EXECUTABLE_PATH: &str = include_str!("../viridithas_executable_path.txt");
pub const MAIA_EXECUTABLE_PATH: &str = include_str!("../maia_executable_path.txt");

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = <cliargs::Cli as clap::Parser>::parse();

    if args.debug {
        log::set_max_level(log::LevelFilter::Debug);
    }

    if args.lichess {
        lichess::main().await;
    }

    if args.engine {
        engine::main();
    }
}
