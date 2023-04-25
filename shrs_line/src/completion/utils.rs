//! Collection of completion functions

use std::path::Path;

// also provide some commonly used completion lists
// - directories
// - executables
// - file extension
// - filename regex
// - known hosts

pub fn cmdname_completer() -> Vec<String> {
    // find_executables_in_path(env.get("PATH").unwrap())
    todo!()
}

pub fn filepath_completer() -> Vec<String> {
    let cur_dir = match std::env::current_dir() {
        Ok(cur_dir) => cur_dir,
        Err(_) => return vec![],
    };
    filepaths(&cur_dir).unwrap_or(vec![])
}

/// Generate list of files in the current working directory with predicate
fn filepaths_p<P>(dir: &Path, predicate: P) -> std::io::Result<Vec<String>>
where
    P: FnMut(&std::fs::DirEntry) -> bool,
{
    use std::fs;

    let out: Vec<String> = fs::read_dir(dir)?
        .filter_map(|f| f.ok())
        .filter(predicate)
        .map(|f| f.file_name().into_string())
        .filter_map(|f| f.ok())
        .collect();

    Ok(out)
}

/// Generate list of files in the current working directory
fn filepaths(dir: &Path) -> std::io::Result<Vec<String>> {
    filepaths_p(dir, |_| true)
}

/// Generate list of all executables in PATH
fn exectuables(_dir: &Path) -> std::io::Result<Vec<String>> {
    todo!()
}

/// Generate list of all ssh hosts
fn ssh_hosts(_dir: &Path) -> std::io::Result<Vec<String>> {
    todo!()
}

/// Looks through each directory in path and finds executables
fn find_executables_in_path(path_str: &str) -> Vec<String> {
    use std::{fs, os::unix::fs::PermissionsExt};

    let mut execs = vec![];
    for path in path_str.split(":") {
        let dir = match fs::read_dir(path) {
            Ok(dir) => dir,
            Err(_) => continue,
        };
        for file in dir {
            if let Ok(dir_entry) = file {
                // check if file is executable
                if dir_entry.metadata().unwrap().permissions().mode() & 0o111 != 0 {
                    execs.push(dir_entry.file_name().to_str().unwrap().into());
                }
            }
        }
    }
    execs
}
