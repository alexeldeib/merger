use std::collections::VecDeque;
use std::ffi::OsString;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
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
    Retry {
        #[arg(short, long, required = true, num_args = 1..)]
        command: Vec<OsString>,
        #[arg(short, long, required = true)]
        timeout: Option<u64>,
        #[arg(short, long, required = true)]
        retries: Option<u64>,
        #[arg(short, long, required = true)]
        wait: Option<u64>,
        #[arg(short, long)]
        max_wait: Option<u64>,
    }
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
        Commands::Retry { mut command, timeout, retries, wait, max_wait } => {
           retry(command, timeout, retries, wait, max_wait);
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

fn retry(command: Vec<OsString>, timeout: Option<u64>, retries: Option<u64>, wait: Option<u64>, max_wait: Option<u64>) {
    // let command_str = command.into_string().unwrap();
    let mut args = VecDeque::from(command);
    let exec = args.pop_front().unwrap();
    println!("exec: '{:#?}'", exec);
    println!("args: '{:#?}'", args);
    let mut cmd = Command::new(exec);
    for arg in args {
        cmd.arg(arg);
    }
    println!("cmd: '{:#?}'", cmd);

    let output = cmd.output().expect("failed to execute process");
    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
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

