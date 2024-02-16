//! Module for the implementation of the native functions offered by the RLox interpreter

pub trait NativeFunctions {
    ///
    /// Returns the current time in seconds since the Unix epoch
    ///
    fn get_time_secs() -> f64;

    ///
    /// Returns the current instant in milliconds since the Unix epoch
    ///
    fn get_instant_msecs() -> f64;
}
