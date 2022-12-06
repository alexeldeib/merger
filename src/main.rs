use std::fs::File;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use clap::{arg, command};
use json_patch::{Patch, from_value};
use serde_json::{json};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(subcommand_required = true)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    JsonPatch {
        #[arg(short, long, required = true)]
        base: PathBuf,
        #[arg(short, long, num_args = 1..)]
        patches: Vec<PathBuf>,
    },
    JsonMerge {
        #[arg(required = true, num_args = 1..)]
        files: Vec<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::JsonPatch { base, patches } => {
            patch(base, patches);
        },
        Commands::JsonMerge { files } => {
            merge(files);
        },
    }
}

fn patch(base: PathBuf, patches: Vec<PathBuf>) {
    let base_file = File::open(base).expect("could not open base file");
    let mut base_json: serde_json::Value = serde_json::from_reader(base_file).expect("base file should be proper JSON");

    let patch_iterator = patches.iter();
    
    for path in patch_iterator {
        let patch_file = File::open(path).expect("could not open path file");
        let patch_json: serde_json::Value = serde_json::from_reader(patch_file).expect("patch file should be proper JSON");
        let patch: Patch = from_value(patch_json).expect("failed to convert patch json to Patch type");

        json_patch::patch(&mut base_json, &patch).expect("failed to patch base json");    
    }

    println!("{}", serde_json::to_string_pretty(&base_json).expect("failed to pretty print final result"));
}

fn merge(files: Vec<PathBuf>) {
    let mut base_json: serde_json::Value = json!({});

    let file_iter = files.iter();
    
    for path in file_iter {
        let patch_file = File::open(path).expect("could not open path file");
        let patch_json: serde_json::Value = serde_json::from_reader(patch_file).expect("merge file should be proper JSON");

        json_patch::merge(&mut base_json, &patch_json);    
    }

    println!("{}", serde_json::to_string_pretty(&base_json).expect("failed to pretty print final result"));
}
    // let matches = command!()
    //     .propagate_version(true)
    //     .subcommand_required(true)
    //     .arg_required_else_help(true)
    //     .subcommand(
    //         Command::new("json-patch")
    //         .arg_required_else_help(true)
    //         .arg(Arg::new("base").short('b').long("base").required(true))
    //         .arg(Arg::new("patch").short('p').long("patch").num_args(1..).action(ArgAction::Append)),
    //     )
    //     .subcommand(
    //         Command::new("json-merge")
    //         .arg_required_else_help(true)
    //         .arg(Arg::new("file").short('f').long("file").num_args(1..).action(ArgAction::Append)),
    //     )
    //     .get_matches();

