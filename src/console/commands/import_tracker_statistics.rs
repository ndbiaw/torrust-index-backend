//! It imports statistics for all torrents from the linked tracker.

use std::env;
use std::sync::Arc;

use derive_more::{Display, Error};
use text_colorizer::*;

use crate::config::Configuration;
use crate::databases::database::connect_database;
use crate::tracker::TrackerService;

const NUMBER_OF_ARGUMENTS: usize = 0;

#[derive(Debug)]
pub struct Arguments {}

#[derive(Debug, Display, PartialEq, Error)]
#[allow(dead_code)]
pub enum ImportError {
    #[display(fmt = "internal server error")]
    WrongNumberOfArgumentsError,
}

fn parse_args() -> Result<Arguments, ImportError> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != NUMBER_OF_ARGUMENTS {
        eprintln!(
            "{} wrong number of arguments: expected {}, got {}",
            "Error".red().bold(),
            NUMBER_OF_ARGUMENTS,
            args.len()
        );
        print_usage();
        return Err(ImportError::WrongNumberOfArgumentsError);
    }

    Ok(Arguments {})
}

fn print_usage() {
    eprintln!(
        "{} - imports torrents statistics from linked tracker.

        cargo run --bin import_tracker_statistics

        ",
        "Tracker Statistics Importer".green()
    );
}

pub async fn run_importer() {
    import(&parse_args().unwrap()).await;
}

pub async fn import(_args: &Arguments) {
    println!("Importing statistics from linked tracker ...");

    let cfg = match Configuration::load_from_file().await {
        Ok(config) => Arc::new(config),
        Err(error) => {
            panic!("{}", error)
        }
    };

    let settings = cfg.settings.read().await;

    let tracker_url = settings.tracker.url.clone();

    eprintln!("Tracker url: {}", tracker_url.green());

    let database = Arc::new(
        connect_database(&settings.database.connect_url)
            .await
            .expect("Database error."),
    );

    let tracker_service = Arc::new(TrackerService::new(cfg.clone(), database.clone()));

    tracker_service.update_torrents().await.unwrap();
}
