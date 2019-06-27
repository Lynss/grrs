use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use exitfailure::ExitFailure;
use failure::ResultExt;

#[derive(StructOpt, Debug)]
pub struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    pub path: PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let content = fs::read_to_string(&args.path)
        .with_context(|_| format!("could not read file `{}`", args.path.display()))?;
    grrs::find_matches(
        &content,
        &args.pattern,
        &mut std::io::stdout(),
    )?;
    Ok(())
}
