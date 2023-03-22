use std::fs::{ File};

use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use clap::{App, Arg};

struct Config {
    target: PathBuf,
    code: String,
    rustfmt_config: Option<PathBuf>,
    rust_version: Option<String>,
}


impl Config {
    fn from_args() -> Self {
        let matches = App::new("Rust code uploader and formatter")
            .arg(
                Arg::with_name("target")
                    .short('t')
                    .long("target")
                    .value_name("FILE")
                    .takes_value(true)
                    .required(true)
                    .help("The file or directory to upload the code to"),
            )
            .arg(
                Arg::with_name("code")
                    .short('c')
                    .long("code")
                    .value_name("CODE")
                    .takes_value(true)
                    .required(true)
                    .help("The code to be uploaded"),
            )
            .arg(
                Arg::with_name("rustfmt-config")
                    .long("rustfmt-config")
                    .value_name("FILE")
                    .takes_value(true)
                    .help("Path to a rustfmt configuration file"),
            )
            .arg(
                Arg::with_name("rust-version")
                    .long("rust-version")
                    .value_name("VERSION")
                    .takes_value(true)
                    .help("The Rust version to use for formatting"),
            )
            .get_matches();

        let target: PathBuf = PathBuf::from(matches.value_of("target").unwrap());
        let code: String = String::from(matches.value_of("code").unwrap());
        let rustfmt_config: Option<PathBuf> = matches.value_of("rustfmt-config").map(PathBuf::from);
        let rust_version: Option<String> = matches.value_of("rust-version").map(String::from);

        Self {
            target,
            code,
            rustfmt_config,
            rust_version,
        }
    }
}

fn main() -> io::Result<()> {
    let config: Config = Config::from_args();

    create_and_write_file(&config.target, &config.code)?;

    run_rustfmt(&config)?;

    println!("Code successfully uploaded and formatted");

    Ok(())
}

fn create_and_write_file(target: &PathBuf, code: &str) -> io::Result<()> {
    if !target.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Target does not exist: {:?}", target),
        ));
    }

    let target: PathBuf = if target.is_dir() {
        target.join("main.rs")
    } else {
        target.to_owned()
    };

    let mut file: File = File::create(&target)?;

    file.write_all(code.as_bytes())?;

    Ok(())
}

fn run_rustfmt(config: &Config) -> io::Result<()> {
    let rustfmt_path: Vec<u8> = Command::new("rustup")
        .arg("which")
        .arg("rustfmt")
        .stderr(Stdio::null())
        .output()
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "Could not find rustfmt in PATH"))?
        .stdout;

    let rustfmt_path: String = String::from_utf8(rustfmt_path)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "rustfmt path contains invalid UTF-8"))?
        .trim()
        .to_owned();

    let mut rustfmt_args: Vec<&str> = vec!["--emit", "files"];

    if let Some(ref rustfmt_config) = config.rustfmt_config {
        rustfmt_args.push("--config-path");
        rustfmt_args.push(rustfmt_config.to_str().unwrap());
    }

    if let Some(ref rust_version) = config.rust_version {
        rustfmt_args.push("--edition");
        rustfmt_args.push(rust_version);
    }

    rustfmt_args.push(config.target.to_str().unwrap());

    let rustfmt_status = Command::new(rustfmt_path)
        .args(&rustfmt_args)
        .status()?;

    if !rustfmt_status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("rustfmt failed with status: {:?}", rustfmt_status),
        ));
    }

    Ok(())
}


