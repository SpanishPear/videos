#![allow(dead_code)]
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use clap::{Parser, ValueHint};
use serde::Deserialize;

#[derive(Debug, Parser)]
#[clap(name = "csv_ops", about = "CSV operations tool")]
struct Opts {
    #[arg(value_hint = ValueHint::FilePath)]
    file: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    #[command(name = "sum")]
    Sum {
        #[arg(long)]
        header: String,
    },
}

#[derive(Deserialize, Debug)]
struct Record {
    name: String,
    age: u32,
    salary: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();

    let file = File::open(opts.file)?;
    let reader = BufReader::new(file);
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(reader);

    match opts.command {
        Command::Sum {  header } => {
            let mut total = 0.0;

            for result in csv_reader.deserialize::<Record>() {
                let record: Record = result?;

                let value = match header.as_str() {
                    "first_name" => 0.0,
                    "last_name" => 0.0,
                    "age" => record.age as f64,
                    "email" => 0.0,
                    "salary" => record.salary,
                    _ => return Err(format!("invalid header: {}", header).into()),
                };


                total += value;

            }

            println!("Total value for column {}: {}", header, total);
        }

    }

    Ok(())
}

