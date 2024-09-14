use anyhow::{Context, Result};
use clap::Parser;
use std::io::Read;
use std::path::PathBuf;

const SHELL_BOILERPLATE: &str = r#"
toml_file=file.toml
toml_keys=()
tt_out=$(mktemp 'tt_out.XXXXXXXXXX'); tt_err=$(mktemp 'tt_err.XXXXXXXXXX')
if tigerturtle $toml_file -- ${toml_keys[@]} >$tt_out 2>$tt_err; then
    eval $(<$tt_out); rm $tt_out; rm $tt_err;
else
    echo "$(<$tt_err)" >&2; rm $tt_out; rm $tt_err; exit 1;
fi
"#;

#[derive(Debug, Parser)]
#[clap(name = "tigerturtle")]
#[clap(about = "Parse and evaluate toml files in bash")]
#[clap(author = "https://ariel.ninja")]
#[clap(version)]
pub struct Args {
    /// Toml file (pass nothing to read from stdin)
    #[arg()]
    pub file: Option<PathBuf>,
    /// Nested delimiter
    #[arg(short = 'd', long, default_value_t = String::from("__"))]
    pub delim: String,
    /// Evaluated variables prefix
    #[arg(short = 'p', long)]
    pub output_prefix: Option<String>,
    /// Default TOML
    #[arg(short = 'D', long)]
    pub default: Option<String>,
    /// Write default TOML if file is missing
    #[arg(short = 'W', long)]
    pub write_missing: bool,
    /// Required key prefix
    #[arg(short = 'r', long, default_value_t = String::from("_"))]
    pub required_prefix: String,
    /// Generate shellscript boilerplate
    #[arg(short = 'G', long)]
    pub generate: bool,
    /// Keys to parse from TOML
    #[arg(raw = true)]
    pub keys: Vec<String>,
}

pub fn run() -> Result<()> {
    let args = Args::parse();
    if args.generate {
        println!("{SHELL_BOILERPLATE}");
        return Ok(());
    }
    if args.write_missing {
        if let Some(default_content) = args.default.as_ref() {
            if let Some(file) = args.file.as_ref() {
                write_default_if_missing(file, default_content).context("write default toml")?;
            }
        }
    }
    let toml_contents = get_toml_content(args.file.as_ref(), args.default).context("get toml")?;
    let evaluation_string = process_toml(
        &toml_contents,
        args.keys,
        &args.output_prefix.unwrap_or_default(),
        &args.required_prefix,
        &args.delim,
    )
    .context("process toml")?;
    println!("{evaluation_string}");
    Ok(())
}

pub fn process_toml(
    toml_contents: &str,
    mut keys: Vec<String>,
    output_prefix: &str,
    required_prefix: &str,
    delim: &str,
) -> Result<String> {
    let parsed_toml: toml::Table = toml::from_str(toml_contents).context("parse toml")?;
    let mut lines = Vec::new();
    for key in &mut keys {
        let required = if let Some(stripped_key) = key.strip_prefix(required_prefix) {
            *key = stripped_key.to_owned();
            true
        } else {
            false
        };
        let key_path: Vec<String> = key
            .split(delim)
            .map(std::borrow::ToOwned::to_owned)
            .collect();
        let bash_key = key_path.join(delim);
        let value = match (required, get_toml_value(&parsed_toml, &key_path)) {
            (true, None) => anyhow::bail!(format!("missing required key: {key}")),
            (false, None) => String::default(),
            (_, Some(v)) => v,
        };
        lines.push(format!("{output_prefix}{bash_key}={value}"));
    }
    Ok(lines.join("\n"))
}

fn write_default_if_missing(file: &PathBuf, default: &String) -> Result<()> {
    if file.exists() {
        return Ok(());
    }
    if let Some(parent) = file.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).context("create directory for default file")?;
        }
    }
    std::fs::write(file, default).context("write default file contents")
}

pub fn get_toml_content(file: Option<&PathBuf>, default: Option<String>) -> Result<String> {
    let toml_contents: String = if let Some(toml_file) = file {
        if toml_file.exists() {
            std::fs::read_to_string(toml_file).context("read file")?
        } else if let Some(default_content) = default {
            default_content
        } else {
            anyhow::bail!("file does not exist and no default provided");
        }
    } else {
        let mut stdin_input = String::new();
        std::io::stdin()
            .read_to_string(&mut stdin_input)
            .context("read stdin")?;
        stdin_input
    };
    Ok(toml_contents)
}

fn get_toml_value(table: &toml::Table, key_path: &[String]) -> Option<String> {
    if let Some(next_key_part) = key_path.first() {
        let next_value_part = table.get(next_key_part)?;
        return match next_value_part {
            toml::Value::Table(inner_table) => get_toml_value(inner_table, key_path.split_at(1).1),
            value => (key_path.len() == 1).then_some(value.to_string()),
        };
    };
    None
}
