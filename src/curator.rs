/// The curator is in charge of searching through files for entries, cataloguing and returning
/// them, as well as potentially modifying entries, and writing them back out to file.
///
///

use std::fs;
use std::path;
use std::string;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::io::Read; // could use std::io::prelude::*


#[derive(Debug)] // Deriving for debug's implementation of the Display trait
pub enum ConstructErr {
    Unknown,
    DoesntExist,
    NotADir,
}

/// construct_file_list custructs a file list in a given directory of all files of a given file
/// extension.
pub fn construct_file_list(dir: &path::PathBuf,
                           file_extension: &string::String)
                           -> Result<VecDeque<path::PathBuf>, ConstructErr> {
    // Check if path exists
    if !dir.exists() {
        return Err(ConstructErr::DoesntExist);
    }

    let mut file_list: VecDeque<path::PathBuf> = VecDeque::new();

    let paths = fs::read_dir(dir).unwrap();
    for p in paths {
        let pval = p.unwrap();

        // Check file type, folder or whatnot
        if pval.path().is_dir() {
            // construct file list.
            let res = construct_file_list(&pval.path(), file_extension);
            match res {
                // merge into current list to be returned.
                Ok(vc) => {
                    file_list.extend(vc);
                }
                Err(evc) => {
                    // Escalate error, or report it, or something.
                    return Err(evc);
                }
            }
        } else {
            // Check file extensions
            match pval.path().extension() {
                Some(ext) => {
                    if ext.to_str() == Some(file_extension) {
                        // Push back path onto some vec
                        file_list.push_back(pval.path());
                    }
                }
                None => { /* TODO: Report error or something*/ }
            }
        }
    }

    Ok(file_list)
}

#[derive(Debug)] // Deriving for debug's implementation of the Display trait
pub enum SearchErr {
    Unknown,
    DoesntExist,
    NotAFile,
}

/// SearchResult contains all instances of a given search string relative to a document, containing
/// its location.
pub struct SearchResult {
    pub path: path::PathBuf,
    pub lines: VecDeque<(u32, String, VecDeque<u32>)>,
}

/// seach_file opens a file reads in its contents, and searches the text.
/// TODO :
///     - streaming search
///         - read in part of the file.
///         - search that bit.
///             - record results.
///         - move on.
pub fn search_file(path: &path::PathBuf, search_str: &str) -> Result<SearchResult, SearchErr> {
    let mut result = SearchResult {
        path: path.clone(),
        lines: VecDeque::<(u32, String, VecDeque<u32>)>::new(),
    };
    // Validate it's existence
    if !path.exists() {
        return Err(SearchErr::DoesntExist);
    }
    // Make sure it's a file
    if !path.is_file() {
        return Err(SearchErr::NotAFile);
    }

    // Open the file
    let file = File::open(path).unwrap();
    // Read in its contents
    let mut buf_reader = BufReader::new(file);
    let mut content_buf = String::new();
    buf_reader.read_to_string(&mut content_buf).unwrap();

    // Split string by line
    let split = content_buf.split("\n");
    let mut line = 0;
    for s in split {
        let lower = s.to_lowercase();
        let search = String::from(search_str.to_lowercase());
        if lower.contains(search.as_str()) {
            let positions = find_positions(lower.as_str(), search.as_str());
            // find position
            // record line.
            // record line number.
            let lt = (line, String::from(s), positions);
            result.lines.push_back(lt);
            // record column position.

        }
        // Increment line
        line += 1;
    }

    // Search Contents


    //  - Record results

    return Ok(result);
}

/// find_position finds all instances of the search string within the line buffer given.
fn find_positions(line_buf: &str, search_str: &str) -> VecDeque<u32> {
    let positions = VecDeque::<u32>::new();


    return positions;
}

// Unit tests
#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    #[test]
    fn construct_file_list_test() {
        // Lets test this on the src directory
        let mut path = env::current_dir().unwrap();
        path.push("src");

        // And use the rs extension to get a list of rust files.
        let v = construct_file_list(&path, &string::String::from("rs"));
        if v.is_ok() {
            for entry in v {
                println!("VecDeque : {:?}\n", entry);
            }
        } else {
            assert!(v.is_ok());
        }
    }

}
