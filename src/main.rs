use std::fs;
use std::path::PathBuf;
use clap::{Parser, Subcommand,};

use interpreter::Lexer;
use miette::{Context, IntoDiagnostic};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Tokenize { filename: PathBuf },
    // Parse { filename: PathBuf },
    // Run { filename: PathBuf },
}


fn main() -> miette::Result<()>{
    let args = Args::parse();
    println!("{:?}", args);

    match args.command {
        Commands::Tokenize { filename } => {
            let file_contents = fs::read_to_string(&filename)
                .into_diagnostic()
                .wrap_err_with(|| format!("reading file failed: {}", filename.display()))?;

            for token in Lexer::new(&file_contents){
                let token = token?;
                println!("{}", token);
            }
        },
       
    }
    Ok(())
}