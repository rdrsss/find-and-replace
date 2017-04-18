

// Integration tests
//
#[cfg(test)]
mod tests {

    use std::fs;
    use std::env;
    use std::path;

    /// get_tests_path will construct a path to the tests directory from the top level
    /// project directory.
    fn get_tests_path() -> path::PathBuf {
        let mut path = env::current_dir().unwrap();
        path.push("tests");
        return path;
    }

    /// setup_test_file copies loremipsum.txt to be modified by the tests.
    fn setup_test_file() {
        let path = get_tests_path();
        println!("{:?}", path);
    }

    #[test]
    fn lorem_ipsum_test() {
        setup_test_file();
        // copy text file to be modified.

        // find and replace instances of lorem
    }

}
