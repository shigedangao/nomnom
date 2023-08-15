use crate::error::Error;

pub struct Logger;

impl Logger {
    /// Display a success message
    ///
    /// # Arguments
    ///
    /// * `S` - msg
    pub fn success<S>(msg: S)
    where
        S: AsRef<str> + std::fmt::Display,
    {
        println!("✅ {msg}");
    }
    /// Display an information message
    ///
    /// # Arguments
    ///
    /// * `msg` - S
    pub fn info<S>(msg: S)
    where
        S: AsRef<str> + std::fmt::Display,
    {
        println!("ℹ️ {msg}")
    }

    /// Display a warning message
    ///
    /// # Arguments
    ///
    /// * `msg` - S
    pub fn warn<S>(msg: S)
    where
        S: AsRef<str> + std::fmt::Display,
    {
        println!("⚠️ {msg}")
    }

    /// Display an error message and terminates the program
    ///
    /// # Arguments
    ///
    /// * `msg` - S
    pub fn error<S>(msg: S, err: Error)
    where
        S: AsRef<str> + std::fmt::Display,
    {
        println!("🟥 Unexpected error: {msg}");

        panic!("{:?}", err.to_string());
    }
}
