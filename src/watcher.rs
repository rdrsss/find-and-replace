
use std::fs;
use std::path;
use std::string;
use std::collections::VecDeque;


pub enum ConstructErr {
    Unknown,
    DoesntExist,
    NotADir,
}

/// construct_file_list custructs a file list in a given directory of all files of a given file
/// extension.
fn construct_file_list(dir: &path::Path,
                       file_extension: &string::String)
                       -> Result<VecDeque<String>, ConstructErr> {
    // Check if path exists
    if dir.exists() != true {
        return Err(ConstructErr::DoesntExist);
    }

    let mut file_list: VecDeque<String> = VecDeque::new();

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
                }
            }
        } else {
            // Check file extensions
            match pval.path().extension() {
                Some(ext) => {
                    if ext.to_str() == Some(file_extension) {
                        // Push back path onto some vec
                        match pval.path().to_str() {
                            Some(path_str) => {
                                file_list.push_back(path_str.to_string());
                            }
                            None => { /*Report error or something*/ }
                        }
                    }
                }
                None => { /*Report error or something*/ }
            }
        }
    }

    Ok(file_list)
}


// Unit tests

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    #[test]
    fn construct_file_list_test() {
        let mut path = env::current_dir().unwrap();
        println!("Current dir : {}", path.display());
        path.push("src");
        println!("Current dir : {}", path.display());

        let v = construct_file_list(&*path, &string::String::from("rs"));
        if v.is_ok() {
            println!("Ok!");
            for entry in v {
                println!("VecDeque : {:?}\n", entry);
            }
        } else {
            assert!(v.is_ok());
        }


    }

}
