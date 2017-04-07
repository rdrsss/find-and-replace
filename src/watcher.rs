use std::fs;
use std::path;
use std::string;
use std::collections::VecDeque;


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
                    //println!("{:?}", evc);
                    warn!("Unable to construct file list {:?}", evc);
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
    NotADir,
}

/// SearchResult contains all instances of a given search string relative to a document, containing
/// its location.
pub struct SearchResult {
    path: path::PathBuf,
}

pub fn search_file(path: &path::PathBuf) -> Result<SearchResult, SearchErr> {
    let mut result = SearchResult { path: path.clone() };

    return Ok(result);
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
