use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;
use anyhow::Context;

#[allow(dead_code)]
pub enum FileNameFilter {
    None,
    CaseSensitive(Vec<String>),
    CaseInsensitive(Vec<String>),
}

pub fn list_files_internal(res: &mut Vec<PathBuf>, path: &PathBuf, file_filter: &FileNameFilter, recurse: bool) -> anyhow::Result<()> {
    let entries = fs::read_dir(path)
        .context(format!("Could not list files from [{}]", path.to_str().unwrap()))?;

    for path in entries.flatten() {
        if let Ok(file_type) = path.file_type() {
            if file_type.is_dir() {
                if recurse {
                    list_files_internal(res, &path.path(), file_filter, recurse)?;
                }
            } else if is_path_valid(file_filter, &path) {
                res.push(path.path());
            }
        }
    }
    Ok(())
}

fn is_path_valid(file_filter: &FileNameFilter, path: &DirEntry) -> bool {
    match file_filter {
        FileNameFilter::None => {
            return true;
        }
        FileNameFilter::CaseSensitive(filters) => {
            if let Some(name) = path.file_name().to_str() {
                for filter in filters.iter() {
                    if name.contains(filter) {
                        return true;
                    }
                }
            }
        }
        FileNameFilter::CaseInsensitive(filters) => {
            let name = path.file_name().to_str().unwrap().to_lowercase();
            for filter in filters.iter() {
                if name.contains(filter) {
                    return true;
                }
            }
        }
    }
    false
}

pub fn list_files(path: &PathBuf, file_filter: &FileNameFilter, recurse: bool) -> anyhow::Result<Vec<PathBuf>> {
    let filter = match file_filter {
        FileNameFilter::None => FileNameFilter::None,
        FileNameFilter::CaseSensitive(s) => FileNameFilter::CaseSensitive(s.clone()),
        FileNameFilter::CaseInsensitive(s) => FileNameFilter::CaseInsensitive(s.iter().map(|x| x.to_lowercase()).collect()),
    };

    let mut dirs: Vec<PathBuf> = vec![];
    list_files_internal(&mut dirs, path, &filter, recurse)?;
    Ok(dirs)
}
