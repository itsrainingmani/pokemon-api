mod db;
mod pokemon_csv;
use db::*;
use pokemon_csv::*;

fn main() -> Result<(), csv::Error> {
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
