# unwrap_or_log
This crate adds functions to all [`std::results::Result<T,E>`] where E implements [`std::error::Error`] by implementing the trait [`Loggable<T>`] for them

The trait [`Loggable<T>`] exposes three functions:

- `unwrap_or_log`: Returns the contained value [`T`] or logs the error and panics
- `log_if_error`: Log the error and returns itself
- `ok_or_log`: Converts [`std::result::Result<T, E>`] to an [`Option<T>`] and logs the error if present