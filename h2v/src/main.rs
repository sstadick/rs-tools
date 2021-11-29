use clap::Parser;
use std::{
    error::Error,
    io::{stdin, stdout, BufWriter, Write},
};

type DynError = Box<dyn Error + 'static>;

/// Rotate a delimited set of input from horizontal to vertical.
///
/// This reads from stdin and writes to stdout.
///
/// Note, this reads the whole input into memory at once.
#[derive(Debug, Parser)]
struct Opts {
    /// The field delimiter (single byte)
    #[clap(long, short, default_value = "\t")]
    delim: String,
}
fn main() -> Result<(), DynError> {
    let opts = Opts::parse();
    let reader = stdin();
    let reader = reader.lock();
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(opts.delim.as_bytes()[0])
        .has_headers(false)
        .from_reader(reader);

    // Collect all the records
    let records: Vec<_> = reader.records().map(|r| r.unwrap()).collect();
    let longest = records.iter().max_by_key(|r| r.len()).map_or(0, |r| r.len());

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for i in 0..longest {
        for (j, row) in records.iter().enumerate() {
            if let Some(value) = row.get(i) {
                writer.write_all(value.as_bytes())?;
            }

            if j != records.len() - 1 {
                writer.write_all(&opts.delim.as_bytes()[0..1])?;
            } else {
                writer.write_all(&[b'\n'])?;
            }
        }
    }
    writer.flush()?;
    Ok(())
}
