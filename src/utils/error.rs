/// # error_if_necessary
///
/// Return Ok value when the result is Ok, otherwise print the error and exit.
///
/// ### Arguments:
/// + result: [std::result::Result]
///
pub fn error_if_necessary<T, E: ToString>(r: std::result::Result<T, E>) -> T {
    match r {
        Ok(ok) => return ok,
        Err(err) => {
            clin::components::error("something went wrong", err.to_string());
            std::process::exit(1);
        }
    }
}
