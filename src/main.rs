// Adjust Compiler Settings
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]


// Import External Crates
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate exec;
extern crate env_logger;
extern crate regex;

// Create Sub-Modules
mod errors;
mod archive;


use clap::App;
use env_logger::LogBuilder;
use errors::*;
use log::{LogLevelFilter, LogRecord};
use std::ffi::OsString;
use std::path::Path;

fn setup_logger(verbosity: u64) {
    let log_level = match verbosity {
        0 => LogLevelFilter::Warn,
        1 => LogLevelFilter::Info,
        2 => LogLevelFilter::Debug,
        _ => LogLevelFilter::Trace,
    };

    let format = |record: &LogRecord| format!("{} - {}", record.level(), record.args());

    let mut builder = LogBuilder::new();
    builder.format(format).filter(None, log_level);

    builder.init().unwrap();
}

fn eprint_cmd(cmd: &[OsString]) {
    for arg in cmd {
        eprint!("{} ", arg.to_string_lossy());
    }
    eprintln!();
}

fn run() -> Result<()> {
    // Read Args / Setup Logging
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    setup_logger(matches.occurrences_of("verbose"));
    trace!("{:?}", matches);

    // Determine proper extraction command
    let user_path = Path::new(matches.value_of("INPUT").unwrap());
    let arc = archive::Archive::from_user_path(user_path)?;
    let cmd = arc.extract_cmd();
    eprint_cmd(&cmd);

    // Exec the command (or not)
    if matches.is_present("dryrun") {
        warn!("Not executing due to '--dry-run'");
        Ok(())
    } else {
        let exec_err = exec::execvp(cmd[0].clone(), cmd);
        bail!(ErrorKind::ExecError(format!("{}", exec_err)));
    }
}

fn main() {
    if let Err(ref e) = run() {
        eprintln!("error: {}", e);

        for e in e.iter().skip(1) {
            eprintln!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            eprintln!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}
