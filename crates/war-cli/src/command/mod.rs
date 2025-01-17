use failure::Error;
use structopt::StructOpt;

mod crc64;
mod dsav;
mod manifest;
mod obsp;
mod sav;
mod worlds;

/// A save viewer/editor for Darksiders.
#[derive(StructOpt)]
pub enum Command {
    /// Calculate Darksiders's CRC-64 for a string.
    ///
    /// Notably, this is the hash used by the `HString` type.
    ///
    /// Darksiders CRC-64 uses a nonstandard lookup table.
    Crc64(crc64::Command),

    /// Commands for working with Darksiders original save files (.dsav)
    Dsav(dsav::Command),

    /// Commands for working with the manifest file (pc.mnfst)
    Manifest(manifest::Command),

    /// Commands for working with the script package (scripts.obsp)
    Obsp(obsp::Command),

    /// Commands for working with Darksiders Warmastered save files (.sav)
    Sav(sav::Command),

    /// Commands for working with the world file (worlds.mnfst)
    Worlds(worlds::Command),
}

impl Command {
    pub fn run(self) -> Result<(), Error> {
        match self {
            Self::Crc64(command) => command.run(),
            Self::Dsav(command) => command.run(),
            Self::Manifest(command) => command.run(),
            Self::Obsp(command) => command.run(),
            Self::Sav(command) => command.run(),
            Self::Worlds(command) => command.run(),
        }
    }
}
