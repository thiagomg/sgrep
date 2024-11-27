mod listing;
mod filter;
mod cmd_line;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use anyhow::Result;

use atty::Stream;
use clap::Parser;
use crate::cmd_line::Args;
use crate::filter::{ContentFilter, filter_stream};
use crate::listing::{FileNameFilter, list_files};

struct Options {
    recurse: bool,
    root_dir: PathBuf,
    content_filters: ContentFilter,
    file_filter: FileNameFilter,
    file_list: Option<Vec<PathBuf>>,
    show_top_lines: usize,
    raw_output: bool,
}

fn run_files(options: &Options) -> Result<()> {
    env::set_current_dir(&options.root_dir)?;

    let paths = if let Some(ref file_list) = options.file_list {
        file_list.clone()
    } else {
        list_files(&PathBuf::from("."), &options.file_filter, options.recurse).unwrap()
    };

    for path in paths.iter() {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                // If the file does not exist, just continue.
                // For patterns, such as *.rs, when no file is found, bash does not expand and
                // passes *.rs instead
                continue
            },
        };
        let reader = BufReader::new(file);
        let file_path = path.to_str().unwrap().to_string();
        filter_stream(reader, &options.content_filters, Some(&file_path), options.show_top_lines, options.raw_output)?;
    }

    Ok(())
}

fn run_stdin(options: &Options) -> Result<()> {
    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin);
    filter_stream(reader, &options.content_filters, None, options.show_top_lines, options.raw_output)?;
    Ok(())
}

fn args_to_option(is_stdin: bool, args: Args) -> Options {
    // If not stdin, all files will be used in search
    let content_filters = match args.case_insensitive {
        true => ContentFilter::CaseInsensitive(args.pattern),
        false => ContentFilter::CaseSensitive(args.pattern),
    };

    let file_filter = if is_stdin {
        FileNameFilter::None
    } else if let Some(file_pattern) = args.file_pattern {
        FileNameFilter::CaseInsensitive(file_pattern)
    } else {
        FileNameFilter::CaseInsensitive(vec!["".to_string()])
    };

    Options {
        recurse: args.recurse,
        root_dir: args.root,
        content_filters,
        file_filter,
        file_list: args.files,
        show_top_lines: args.show_top.unwrap_or(0),
        raw_output: args.raw,
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let is_stdin = !atty::is(Stream::Stdin);
    let options = args_to_option(is_stdin, args);

    if is_stdin {
        run_stdin(&options).expect("Error reading from stdin");
    } else {
        run_files(&options).expect("Error reading from files");
    }

    Ok(())
}
