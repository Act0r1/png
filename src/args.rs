use clap::Parser;
use clap::Subcommand;
/// Pngme CLI
#[derive(Parser)]
#[clap(name = "png")]
#[clap(about = "A CLI to encode/decode messages in PNG files")]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand)]
pub enum CliCommand {
    /// Encodes message in png file
    Encode {
        #[clap(required = true)]
        file_path: String,
        #[clap(required = true)]
        message: String,
        #[clap(required = false)]
        output_file: Option<String>,
    },
    /// Get message of chunk_type from png at file_path
    Decode {
        #[clap(required = true)]
        file_path: String,
    },
    /// Remove message of chunk_type from png at file_path
    Remove {
        #[clap(required = true)]
        file_path: String,
    },
    Print {
        #[clap(required = true)]
        file_path: String,
    },
}
