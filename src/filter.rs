use std::io::{BufRead, BufReader};
use colored::Colorize;

pub enum ContentFilter {
    CaseSensitive(Vec<String>),
    CaseInsensitive(Vec<String>),
}

pub fn colorize(line: &str, filters: &[String], colored_filters: &[String]) -> String {
    if filters.len() != colored_filters.len() {
        panic!("Internal error. Filter size is invalid");
    }

    let mut res= line.to_string();
    for (pos, filter) in filters.iter().enumerate() {
        res = res.replace(filter, &colored_filters[pos]);
    }

    res
}

fn colorize_filter(filters: &[String]) -> Vec<String> {
    let mut colored = vec![];
    for filter in filters.iter() {
        colored.push(format!("{}", filter.green()));
    }
    colored
}

pub fn filter_stream<R>(reader: BufReader<R>, content_filters: &ContentFilter, prefix: Option<&String>, show_top_lines: usize, raw_output: bool) -> anyhow::Result<()>
where R: std::io::Read {
    let content_filters = match content_filters {
        ContentFilter::CaseSensitive(filters) => ContentFilter::CaseSensitive(filters.clone()),
        ContentFilter::CaseInsensitive(filters) => {
            let items: Vec<String> = filters.iter().map(|f| f.to_lowercase()).collect();
            ContentFilter::CaseInsensitive(items)
        }
    };

    let colored = match &content_filters {
        ContentFilter::CaseSensitive(filters) => {
            colorize_filter(filters)
        }
        ContentFilter::CaseInsensitive(filters) => {
            colorize_filter(filters)
        }
    };

    let mut header = false;
    for (num, line) in reader.lines().enumerate() {
        let line = line?;

        let mut print_matching_lines = |line_to_filter: &String, filters: &Vec<String>, colored: &Vec<String>, force: bool | {
            for filter in filters.iter() {
                if force || line_to_filter.contains(filter) {
                    if raw_output {
                        println!("{}", line);
                    } else {
                        if !header {
                            header = true;
                            if let Some(file_name) = prefix {
                                println!("{}", file_name.purple());
                            }
                        }

                        let line_num = format!("{}", num);
                        println!("{:>4}: {}", line_num.blue(), colorize(&line, filters, colored));
                    }
                }
            }
        };

        let mut force = false;
        if num < show_top_lines  {
            force = true;
        }

        match &content_filters {
            ContentFilter::CaseSensitive(filters) => {
                print_matching_lines(&line, filters, &colored, force);
            }
            ContentFilter::CaseInsensitive(filters) => {
                let low_line = line.to_lowercase();
                print_matching_lines(&low_line, filters, &colored, force);
            }
        };
    }
    if header {
        if let Some(_file_name) = prefix {
            println!();
        }
    }
    Ok(())
}
