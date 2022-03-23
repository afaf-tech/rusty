#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String>{
        if 2 + 2 == 4 {
            Ok(())
        }else { 
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn it_works2() {
        assert_eq!(2 + 2, 4);
    }
}

// by default all test run in pararel in separated threads
// cargo test


// but we can also specify option to run test in one threads
// cargo test -- --test-threads=1