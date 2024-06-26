use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[clap(
    after_help = r##"Examples:
# Displays lines containing "pub struct" string in all .rs files in the local directory
sgrep -p "pub struct" *.rs

# Displays lines containing "#ifdef" or "#ifndef" in all .c and .h files
sgrep -p "#ifdef" -p "#ifndef" -f .c -f .h
"##)]
pub struct Args {
    /// Recursively search
    #[arg(short, long)]
    pub recurse: bool,

    /// Case-insensitive search
    #[arg(short = 'i', long)]
    pub case_insensitive: bool,

    /// Root directory to search
    #[arg(long, default_value = ".")]
    pub root: PathBuf,

    /// Patterns to filter lines in a buffer
    #[arg(short, long, required = true)]
    pub pattern: Vec<String>,

    /// Patterns to filter files. E.g. .cpp, .h, my_class
    #[arg(short)]
    pub file_pattern: Option<Vec<String>>,

    /// Optionally list of files. Otherwise, all files will be searched
    pub files: Option<Vec<PathBuf>>,
}
