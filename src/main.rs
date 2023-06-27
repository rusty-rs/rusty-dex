use clap::{ Parser, Subcommand, Args };

extern crate dex_parser;

use dex_parser::logging;
use dex_parser::{ info, die };

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
    /// Disassemble the whole app or a specific class/method
    Disasm(DisasmArgs), /* {
        // TODO might be cool to recreate the app structure
        // when disassembling the full app
        // TODO disassemble only a class/method
        /// Disassembly options
        #[arg(short, long)]
        output: bool,
    }, */
    /// Get the list of class names in the app
    Classes(ClassesArgs),
    /// Get the list of methods in the app
    Methods(MethodsArgs),
}

#[derive(Args, Debug)]
struct DisasmArgs {
    /// Save disassembly output to this folder
    #[arg(short, long)]
    output: Option<String>,
    /// Only disassemble the specific class(es)
    #[arg(short, long)]
    class_names: Option<Vec<String>>,
    /// Only disassemble the specific method(s).
    /// Works best in conjunction with `--classes`
    #[arg(short, long)]
    method_names: Option<Vec<String>>,
}

#[derive(Args, Debug)]
struct ClassesArgs {
    /// Only show class names starting with this prefix
    #[arg(short, long)]
    prefix: Option<String>
}

#[derive(Args, Debug)]
struct MethodsArgs {
    /// Only show methods from classes which names start with this prefix
    #[arg(short, long)]
    class_prefix: Option<String>,
    /// Only show method names starting with this prefix
    #[arg(short, long)]
    method_prefix: Option<String>
}

fn main() {
    let cli_args = CliArgs::parse();

    let cmd = match cli_args.cmd {
        Some(cmd) => cmd,
        None => {
            die!("No command supplied");
            // Commands::Disasm(DisasmArgs { output: None })
        }
    };

    logging::set_log_level(cli_args.log_level);
    info!("Set log level to {}", cli_args.log_level);

    info!("Loading classes.dex from {}", cli_args.apk);
    let dex_reader = DexReader::build_from_file(&cli_args.apk);

    info!("Parsing DEX file");
    let dex_file = DexFile::build(dex_reader);

    match cmd {
        Commands::Disasm(arg) => dex_file.disasm(arg.output,
                                                 arg.class_names,
                                                 arg.method_names),
        Commands::Classes(arg) => dex_file.print_classes_with_prefix(arg.prefix),
        Commands::Methods(arg) => dex_file.get_methods(arg.class_prefix,
                                                       arg.method_prefix),
        _ => todo!("foo"),
    }
}
