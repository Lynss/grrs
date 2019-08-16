use exitfailure::ExitFailure;
use failure::ResultExt;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
pub struct Cli {
    #[structopt(short = "p", help = "Search pattern")]
    pattern: String,
    #[structopt(name = "file_path", parse(from_os_str), help = "File path")]
    pub path: PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let content = fs::read_to_string(&args.path)
        .with_context(|_| format!("could not read file `{}`", args.path.display()))?;
    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout())?;
    Ok(())
}
