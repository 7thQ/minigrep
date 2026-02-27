


use std::env;
use minigrep::Args;
use std::process;




fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = Args::new(&args).unwrap_or_else(|err| { eprintln!("Problem parsing the arguments: {err}"); process::exit(2);});
    if let Err(e) = Args::run(&arguments) {
        eprintln!("Run fn Error returned: {e}");
        process::exit(4)
    }


}

