use clap::{ Parser, Subcommand, Args };

extern crate dex_parser;

use dex_parser::logging;
use dex_parser::{ info };

use dex_parser::dex_reader::DexReader;
use dex_parser::dex_file::DexFile;

/// DEX parser or some cool other name
#[derive(Parser)]
struct CliArgs {
    /// The path to the file to read
    #[arg(short, long)]
    apk: String,

    /// Log level, 0 (errors only) to 3 (debug messages)
    #[arg(short, long, default_value_t = 0)]
    log_level: u8,

    /// Command to run (default: `disasm`)
    #[command(subcommand)]
    cmd: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Disassemble the whole app
    Disasm, /* {
        // TODO might be cool to recreate the app structure
        // when disassembling the full app
        // TODO disassemble only a class/method
        /// Disassembly options
        #[arg(short, long)]
        output: bool,
    }, */
    /// Get the list of class names in the app
    Classes(PrefixArg),
    /// Get the list of methods in the app
    Methods,  // TODO: get methods only for a specific class
}

#[derive(Args, Debug)]
struct PrefixArg {
    /// Only show names starting with this prefix
    prefix: Option<String>
}

fn main() {
    let cli_args = CliArgs::parse();

    let cmd = match cli_args.cmd {
        Some(cmd) => cmd,
        None => {
            info!("No command supplied: defaulting to `disasm`");
            Commands::Disasm
        }
    };

    logging::set_log_level(cli_args.log_level);
    info!("Set log level to {}", cli_args.log_level);

    info!("Loading classes.dex from {}", cli_args.apk);
    let dex_reader = DexReader::build_from_file(&cli_args.apk);

    info!("Parsing DEX file");
    let dex_file = DexFile::build(dex_reader);

    match cmd {
        Commands::Disasm => dex_file.disasm(),
        Commands::Classes(arg) => dex_file.get_classes(arg.prefix),
        _ => todo!("foo"),
    }
}
