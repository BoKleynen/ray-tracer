use clap::Parser;
use std::num::NonZeroU8;
use tracing::Level;

#[derive(Parser, Debug)]
#[clap(version)]
struct Cli {
    filenames: Vec<String>,

    /// Use specified number of threads for rendering.
    #[clap(long, value_name = "num")]
    nthreads: Option<NonZeroU8>,

    /// Write the final image to the given filename.
    #[clap(long, value_name = "FILE")]
    outfile: Option<String>,

    /// Automatically reduce a number of quality settings to
    /// render more quickly.
    #[clap(long)]
    quick: bool,

    /// Suppress all text output other than error messages.
    #[clap(long)]
    quiet: bool,

    /// Sets the log level, possible values are:
    /// TRACE, DEBUG, INFO, WARN, ERROR.
    /// Default value is INFO
    #[clap(long)]
    log_level: Option<String>,
}

fn main() {
    let cli: Cli = Cli::parse();

    if !cli.quiet {
        tracing_subscriber::fmt()
            .with_max_level(
                cli.log_level
                    .unwrap_or_default()
                    .parse::<Level>()
                    .unwrap_or(Level::INFO),
            )
            .init();
    }

    println!("{:?}", cli.filenames)
}
