use std::io::{BufRead, BufReader};
use colored::Colorize;

pub enum ContentFilter {
    CaseSensitive(Vec<String>),
    CaseInsensitive(Vec<String>),
}

pub fn colorize(line: &str, filters: &Vec<String>, colored_filters: &Vec<String>) -> String {
    if filters.len() != colored_filters.len() {
        panic!("Internal error. Filter size is invalid");
    }

    let mut res= line.to_string();
    for (pos, filter) in filters.iter().enumerate() {
        res = res.replace(filter, &colored_filters[pos]);
    }

    return res;
}

fn colorize_filter(filters: &Vec<String>) -> Vec<String> {
    let mut colored = vec![];
    for filter in filters.iter() {
        colored.push(format!("{}", filter.green()));
    }
    colored
}

pub fn filter_stream<R>(reader: BufReader<R>, content_filters: &ContentFilter, prefix: Option<&String>) -> anyhow::Result<()>
where R: std::io::Read {
    let content_filters = match content_filters {
        ContentFilter::CaseSensitive(filters) => ContentFilter::CaseSensitive(filters.clone()),
        ContentFilter::CaseInsensitive(filters) => {
            let items: Vec<String> = filters.iter().map(|f| f.to_lowercase()).collect();
            ContentFilter::CaseInsensitive(items)
        }
    };

    let mut header = false;
    for (num, line) in reader.lines().enumerate() {
        let line = line?;

        let mut print_matching_lines = |line_to_filter: &String, filters: &Vec<String>, colored: &Vec<String> | {
            for filter in filters.iter() {
                if line_to_filter.contains(filter) {
                    if !header {
                        header = true;
                        if let Some(file_name) = prefix {
                            println!("{}", file_name.purple());
                        }
                    }

                    let line_num = format!("{}", num);
                    println!("{}: {}", line_num.blue(), colorize(&line, filters, colored));
                }
            }
        };

        match &content_filters {
            ContentFilter::CaseSensitive(filters) => {
                let colored = colorize_filter(filters);
                print_matching_lines(&line, filters, &colored);
            }
            ContentFilter::CaseInsensitive(filters) => {
                let colored = colorize_filter(filters);
                let low_line = line.to_lowercase();
                print_matching_lines(&low_line, filters, &colored);
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
