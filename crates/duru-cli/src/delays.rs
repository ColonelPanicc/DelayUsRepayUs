use analyse::Analyse;
use anyhow::Result;
use clap::{Args, Subcommand};

mod analyse;

#[derive(Args)]
pub struct DelaysCommand {
    #[command(subcommand)]
    pub command: DelayCommands,
}

#[derive(Subcommand)]
pub enum DelayCommands {
    /// Analyse NWR Delay Attribution data for delay information.
    Analyse(Analyse),
}

impl DelaysCommand {
    pub fn execute(self, dry_run: bool) -> Result<()> {
        match self.command {
            DelayCommands::Analyse(cmd) => cmd.execute(dry_run),
        }
    }
}
