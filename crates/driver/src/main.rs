use err::catch_fatal_error_marker;
use sess::Session;
use std::process::ExitCode;

fn run() {
    let sess = Session::new(filename);
    let unit = parse(&sess);
}

fn main() -> ExitCode {
    catch_fatal_error_marker(run)
}
