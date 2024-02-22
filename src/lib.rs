use log::error;

///Trait exposing the unwrap_or_log interface
pub trait Loggable<T> {
    ///This function should try to unwrap `self` and if it fails it logs the error and panic
    fn unwrap_or_log(self) -> T;
    ///This function should check if self is an error and if so logs the error then returns the initial value
    fn log_if_error(self) -> Self;
    ///This function should convert `self` to an [`Option<T>`] and log if it contained an error
    fn ok_or_log(self) -> Option<T>;
}

impl<T, E: std::error::Error> Loggable<T> for std::result::Result<T, E> {
    ///Returns the contained value [`T`] or logs the error and panics
    fn unwrap_or_log(self) -> T {
        self.unwrap_or_else(|error| {
            error!("{error}");
            panic!("{error}")
        })
    }

    ///Log the error and returns itself
    fn log_if_error(self) -> Self {
        if let Some(error) = self.as_ref().err() {
            error!("{error}")
        }
        self
    }

    ///Converts [`std::result::Result<T, E>`] to an [`Option<T>`] and logs the error if present
    fn ok_or_log(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                error!("{error}");
                None
            }
        }
    }
}

#[cfg(test)]
mod testing {
    use crate::Loggable;
    use simple_logger::SimpleLogger;
    use std::error::Error;
    use std::fmt::{Display, Formatter};
    use std::panic::set_hook;
    use std::thread::spawn;

    #[derive(Default, Debug, PartialOrd, PartialEq)]
    struct TestError {}

    impl Display for TestError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str("This is a test error")
        }
    }

    impl Error for TestError {}

    type TestResult<T> = std::result::Result<T, TestError>;

    #[test]
    fn test_unwrap_or_log_ok() {
        let _ = SimpleLogger::default().init();
        let data = 42;
        let process = spawn(move || {
            let value = TestResult::Ok(data).unwrap_or_log();
            value
        });

        let join_result = process.join().unwrap();
        assert_eq!(join_result, data);
    }

    #[test]
    fn test_unwrap_or_log_error() {
        let _ = SimpleLogger::default().init();
        set_hook(Box::new(|_| {}));
        let process = spawn(move || {
            let value: i32 = TestResult::Err(TestError::default()).unwrap_or_log();
            value
        });

        let join_result = process.join();
        assert!(join_result.is_err())
    }

    #[test]
    fn test_log_if_error_ok() {
        let _ = SimpleLogger::default().init();
        let data = 42;
        let result = TestResult::Ok(data).log_if_error();
        assert_eq!(result, Ok(data))
    }

    #[test]
    fn test_log_if_error_error() {
        let _ = SimpleLogger::default().init();
        let result: TestResult<i32> = TestResult::Err(TestError::default()).log_if_error();
        assert_eq!(result, Err(TestError::default()))
    }

    #[test]
    fn ok_or_log_ok() {
        let _ = SimpleLogger::default().init();
        let data = 42;
        let result = TestResult::Ok(data).ok_or_log();
        assert_eq!(result, Some(data))
    }

    #[test]
    fn ok_or_log_error() {
        let _ = SimpleLogger::default().init();
        let result: Option<i32> = TestResult::Err(TestError::default()).ok_or_log();
        assert_eq!(result, None)
    }
}
