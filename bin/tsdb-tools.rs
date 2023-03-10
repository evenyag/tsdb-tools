//! TSDB utilities.

use clap::Parser;
use tsdb_tools::influx::InfluxCommand;

/// TSDB utilities.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Command {
    #[clap(subcommand)]
    subcmd: Subcommand,
}

/// Subcommand of TSDB utilities.
#[derive(Debug, Parser)]
enum Subcommand {
    /// Subcommand for InfluxDB target.
    Influx(InfluxCommand),
}

fn main() {
    let cmd = Command::parse();

    match cmd.subcmd {
        Subcommand::Influx(influx) => influx.run(),
    }
}
