mod db;
mod pokemon_csv;
use color_eyre::{eyre, eyre::WrapErr, Section};
use std::env;

use db::*;
use pokemon_csv::*;

fn main() -> eyre::Result<()> {
    color_eyre::install()?; // bootstrap color eyre first so it can show errors
    let database_url = env::var("DATABASE_URL")
        .wrap_err("Must have a DATABASE_URL set")
        .suggestion("Run pscale connect <database> <branch> to get a connection string")?;

    // Since the main function's return type is the same as the error type,
    // we can use ? on the Result<Reader<File>, csv::Error> to turn into a Reader<File>
    let mut rdr = csv::Reader::from_path("./crates/upload-pokemon-data/pokemon.csv")?;

    for result in rdr.deserialize() {
        let record: PokemonCsv = result?;
        let pokemon_row: PokemonTableRow = record.into();
        println!("{:?}", pokemon_row);
    }

    dbg!(PokemonId::new());

    Ok(())
}
