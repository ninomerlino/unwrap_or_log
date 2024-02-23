# unwrap_or_log
This crate adds a function to all [`std::results::Result<T,E>`] where E implements [`std::error::Error`] by implementing the trait [`LogError`] for them

The trait [`LogError`] exposes a function [`LogError::log_error`] that takes a result, logs it if it's an error and returns it unmodified: