//!# unwrap_or_log
//! This crate adds a function to all [`std::results::Result<T,E>`] where E implements [`std::error::Error`] by implementing the trait [`LogError`] for them
//!
//! The trait [`LogError`] exposes a function [`LogError::log_error`] that takes a result, logs it if it's an error and returns it unmodified:

use log::error;
///Trait exposing the  interface
pub trait LogError {
    ///This function should check if self is an error and if so logs the error then returns the initial value
    fn log_error(self) -> Self;
}

impl<T, E: std::error::Error> LogError for std::result::Result<T, E> {
    ///Log the error and returns itself
    fn log_error(self) -> Self {
        if let Some(error) = self.as_ref().err() {
            error!("{error}");
        }
        self
    }
}

#[cfg(test)]
mod testing {
    use crate::LogError;
    use simple_logger::SimpleLogger;
    use std::panic::set_hook;
    use std::thread::spawn;
    use thiserror::Error;

    #[derive(Error, Debug, PartialOrd, PartialEq)]
    enum TestError {
        #[error("Unwrap or Log")]
        UnwrapOrLog,
        #[error("Log if error")]
        LogIfError,
        #[error("Ok or Log")]
        OkOrLogError,
    }

    type TestResult<T> = std::result::Result<T, TestError>;

    #[test]
    fn test_unwrap_or_log_ok() {
        let _ = SimpleLogger::default().init();
        let data = 42;
        let process = spawn(move || {
            let value = TestResult::Ok(data).log_error().unwrap();
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
            let value: i32 = TestResult::Err(TestError::UnwrapOrLog).log_error().unwrap();
            value
        });

        let join_result = process.join();
        assert!(join_result.is_err())
    }

    #[test]
    fn test_log_if_error_ok() {
        let _ = SimpleLogger::default().init();
        let data = 42;
        let result = TestResult::Ok(data).log_error();
        assert_eq!(result, Ok(data))
    }

    #[test]
    fn test_log_if_error_error() {
        let _ = SimpleLogger::default().init();
        let result: TestResult<i32> = TestResult::Err(TestError::LogIfError).log_error();
        assert_eq!(result, Err(TestError::LogIfError))
    }

    #[test]
    fn ok_or_log_ok() {
        let _ = SimpleLogger::default().init();
        let data = 42;
        let result = TestResult::Ok(data).log_error().ok();
        assert_eq!(result, Some(data))
    }

    #[test]
    fn ok_or_log_error() {
        let _ = SimpleLogger::default().init();
        let result: Option<i32> = TestResult::Err(TestError::OkOrLogError).log_error().ok();
        assert_eq!(result, None)
    }
}
