use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[clap(
    after_help = r##"Examples:
# Special usage: If only one positional argument is provided, recursively searches for the provided argument in all files or stdin, case insensitive.
# To search in all files:
sgrep armaria
# To filter stdin
cat my-file | sgrep armaria

# Displays lines containing "pub struct" string in all .rs files in the local directory
sgrep -p "pub struct" *.rs

# Displays lines containing "#ifdef" or "#ifndef" in all .c and .h files
sgrep -p "#ifdef" -p "#ifndef" -f .c -f .h

# Display the top 1 line and filter for bash, case insensitive
ps -ef | sgrep -t 1 -i -p bash
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

    /// Show top n lines
    #[arg(short = 't', long)]
    pub show_top: Option<usize>,

    /// Patterns to filter lines in a buffer
    #[arg(short, long)]
    pub pattern: Option<Vec<String>>,

    /// Excludes lines using this pattern in a buffer
    #[arg(short, long)]
    pub exclude: Option<Vec<String>>,

    /// Patterns to filter files. E.g. .cpp, .h, my_class
    #[arg(short)]
    pub file_pattern: Option<Vec<String>>,

    /// Optionally list of files. Otherwise, all files will be searched
    pub files: Option<Vec<PathBuf>>,
    
    /// Show raw outputs (without colour, headers or line numbers)
    #[arg(long, default_value_t=false)]
    pub raw: bool,
}
