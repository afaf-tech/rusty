fn prints_and_returns_10(a : i32) -> i32 {
    println!("I got the value {}", a);
    10
}
#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn this_test_will_pas() {
        let value = prints_and_returns_10(4);
        assert_eq!(value,10);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5,value);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        assert_eq!(2 + 2, 4);
    }
}

// by default all test run in pararel in separated threads
// cargo test


// but we can also specify option to run test in one threads
// cargo test -- --test-threads=1


// show more detail output
// cargot test -- --show-output

// run one test 
// cargo test testName

// run one module test
// cargo test moduleName::

// only run the ignored test
// cargo test -- --ignored