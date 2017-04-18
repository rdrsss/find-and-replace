// Integration tests
//
#[cfg(test)]
mod tests {
    extern crate curator;

    use std::fs;
    use std::env;
    use std::path;


    const TEST_FILE_NAME: &'static str = "loremipsum.txt";
    const TEST_COPY_NAME: &'static str = "copy_ipsum.txt";

    // get_tests_dir_path will construct a path to the tests directory from the top level
    // project directory.
    fn get_tests_dir_path() -> path::PathBuf {
        let mut path = env::current_dir().unwrap();
        path.push("tests");
        return path;
    }

    // get_test_file_path returens a PathBuf to the test file.
    fn get_test_file_path() -> path::PathBuf {
        let tests_path = get_tests_dir_path();
        let mut file_path = tests_path;
        file_path.push(TEST_FILE_NAME);
        return file_path;
    }

    // get_test_copy_path returens a PathBuf to the test file copy.
    fn get_test_copy_path() -> path::PathBuf {
        let tests_path = get_tests_dir_path();
        let mut file_path = tests_path;
        file_path.push(TEST_COPY_NAME);
        return file_path;
    }

    // setup_test_file copies loremipsum.txt to be modified by the tests.
    fn setup_test_file() -> bool {
        let file_path = get_test_file_path();
        let copy_path = get_test_copy_path();

        let res = fs::copy(file_path, copy_path);
        match res {
            Ok(_) => {}
            Err(evc) => {
                println!("{:?}", evc);
                return false;
            }
        }
        return true;
    }

    // delete_test_file delete's the test file that was created if it exists.
    fn delete_test_file() -> bool {
        let copy_path = get_test_copy_path();

        if !copy_path.exists() {
            return false;
        }

        let res = fs::remove_file(copy_path);
        match res {
            Ok(_) => {}
            Err(rerr) => {
                println!("{:?}", rerr);
                return false;
            }
        }

        return true;
    }

    #[test]
    fn search_test() {
        // Find instances
        let mut lower_count: usize = 0;
        let lower_res = curator::search_file(&get_test_file_path(), "lorem");
        match lower_res {
            Ok(sr) => {
                // Verify search results.
                println!("{:?}", sr.path);
                println!("{:?}", sr.lines.len());
                lower_count = sr.lines.len()
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
        // Test case sensitivity
        let mut mixed_count: usize = 0;
        let mixed_res = curator::search_file(&get_test_file_path(), "LOrem");
        match mixed_res {
            Ok(sr) => {
                // Verify search results.
                println!("{:?}", sr.path);
                println!("{:?}", sr.lines.len());
                mixed_count = sr.lines.len();
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }

        assert_eq!(lower_count, mixed_count);
    }

    #[test]
    fn find_and_replace_test() {
        // Setup test file for finding and replacing.
        setup_test_file();


        // Delete test file.
        delete_test_file();

    }

}
