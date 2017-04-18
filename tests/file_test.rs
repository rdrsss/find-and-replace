

// Integration tests
//
#[cfg(test)]
mod tests {

    use std::fs;
    use std::env;
    use std::path;

    const TEST_FILE_NAME: &'static str = "loremipsum.txt";
    const TEST_COPY_NAME: &'static str = "copy_ipsum.txt";

    /// get_tests_path will construct a path to the tests directory from the top level
    /// project directory.
    fn get_tests_path() -> path::PathBuf {
        let mut path = env::current_dir().unwrap();
        path.push("tests");
        return path;
    }

    /// setup_test_file copies loremipsum.txt to be modified by the tests.
    fn setup_test_file() -> bool {
        let tests_path = get_tests_path();

        let mut file_path = tests_path.clone();
        file_path.push(TEST_FILE_NAME);

        let mut copy_path = tests_path.clone();
        copy_path.push(TEST_COPY_NAME);

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

    /// delete_test_file delete's the test file that was created if it exists.
    fn delete_test_file() -> bool {
        let tests_path = get_tests_path();
        let mut copy_path = tests_path.clone();
        copy_path.push(TEST_COPY_NAME);

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
    fn lorem_ipsum_test() {
        // Setup test file for finding and replacing.
        setup_test_file();

        // Find and replace instances of lorem.

        // Delete test file.
        delete_test_file();
    }

}
