mod listing;
mod filter;
mod cmd_line;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use anyhow::Result;

use atty::Stream;
use clap::{CommandFactory, Parser};
use crate::cmd_line::Args;
use crate::filter::{ContentFilter, filter_stream};
use crate::listing::{FileNameFilter, list_files};

struct Options {
    recurse: bool,
    root_dir: PathBuf,
    content_filters: Option<ContentFilter>,
    content_exclude: Option<ContentFilter>,
    file_filter: FileNameFilter,
    file_list: Option<Vec<PathBuf>>,
    show_top_lines: usize,
    show_line_number: bool,
    raw_output: bool,
    debug_output: bool,
}

fn run_files(options: &Options) -> Result<()> {
    env::set_current_dir(&options.root_dir)?;

    let paths = if let Some(ref file_list) = options.file_list {
        file_list.clone()
    } else {
        list_files(&PathBuf::from("."), &options.file_filter, options.recurse).unwrap()
    };

    for path in paths.iter() {
        if !path.is_file() {
            continue;
        }

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

        if let Err(e) = filter_stream(reader, &options.content_filters, &options.content_exclude, Some(&file_path), options.show_top_lines, options.show_line_number, options.raw_output) {
            if options.debug_output {
                eprintln!("Error ({}): {}", path.display(), e);
            }
        }
    }

    Ok(())
}

fn run_stdin(options: &Options) -> Result<()> {
    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin);
    filter_stream(reader, &options.content_filters, &options.content_exclude, None, options.show_top_lines, options.show_line_number, options.raw_output)?;
    Ok(())
}

fn args_to_option(is_stdin: bool, args: Args) -> Options {
    // If not stdin, all files will be used in search
    let show_line_number: bool = !is_stdin;

    let content_filters = match (args.pattern, args.case_insensitive) {
        (Some(patt), true) => Some(ContentFilter::CaseInsensitive(patt)),
        (Some(patt), false) => Some(ContentFilter::CaseSensitive(patt)),
        (None, _) => None,
    };

    let content_exclude = match (args.exclude, args.case_insensitive) {
        (Some(patt), true) => Some(ContentFilter::CaseInsensitive(patt)),
        (Some(patt), false) => Some(ContentFilter::CaseSensitive(patt)),
        (None, _) => None,
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
        content_exclude,
        file_filter,
        file_list: args.files,
        show_top_lines: args.show_top.unwrap_or(0),
        show_line_number,
        raw_output: args.raw,
        debug_output: false,
    }
}

fn option_for_single_arg(pattern: String, is_stdin: bool) -> Options {
    let content_filters = Some(ContentFilter::CaseInsensitive(vec![pattern]));
    let content_exclude = None;

    let file_filter = if is_stdin {
        FileNameFilter::None
    } else {
        FileNameFilter::CaseInsensitive(vec!["".to_string()])
    };

    let show_line_number: bool = !is_stdin;

    Options {
        recurse: true,
        root_dir: PathBuf::from("."),
        content_filters,
        content_exclude,
        file_filter,
        file_list: None,
        show_top_lines: 0,
        show_line_number,
        raw_output: false,
        debug_output: false,
    }
}

fn main() -> Result<()> {
    let is_stdin = !atty::is(Stream::Stdin);

    let mut raw_args: Vec<String> = std::env::args().skip(1).collect();
    let simpler: bool = if raw_args.len() == 1 {
        let pattern = raw_args.get(0).unwrap();
        if pattern.starts_with('-') {
            false
        } else {
            true
        }
    } else {
        false
    };

    let options: Options = if simpler {
        let pattern = raw_args.remove(0);
        option_for_single_arg(pattern, is_stdin)
    } else {
        let args = Args::parse();

        if args.pattern.is_none() && args.exclude.is_none() {
            Args::command().print_help()?;
            return Ok(());
        }

        args_to_option(is_stdin, args)
    };

    if is_stdin {
        run_stdin(&options).expect("Error reading from stdin");
    } else {
        run_files(&options).expect("Error reading from files");
    }

    Ok(())
}
