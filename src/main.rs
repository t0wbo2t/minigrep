/* To read the command line arguments, we'll need the [std::env::args] function. The first arguement is always the name of the binary.
 * This function returns an iterator of the command line arguments passed to the program we are running.
 *
 * Note that std::env::args will panic if any argument contains invalid Unicode. If your program needs to accept arguments containing
 * invalid Unicode, use std::env::args_os instead.
 */
use std::error::Error;
use std::{fs, env, process};

use minigrep::{search, case_ignore_search};

// We will group the correlated data in a struct to increase cohesion.
pub struct Config {
    pub query:       String,
    pub file_path:   String,
    pub ignore_case: bool,
 }

impl Config {
    // Separate argument parsing from the program logic.
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(query) => query,
            None => return Err("Did not get a query string."),
        };

        let file_path = match args.next() {
            Some(file_path) => file_path,
            None => return Err("Did not get a path to file."),
        };

        // To set ignore_case's value, we call the env::var function and pass it the name of the IGNORE_CASE environment variable.
        // The env::var function returns a Result that will be the successful Ok variant that contains the value of the env variable
        // if the environment variable is set to any value. It will return the Err variant if the environment variable is not set.
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query, file_path, ignore_case,
        })
    }
}

// This Ok(()) syntax might look a bit strange at first. But using () like this is the idiomatic way to indicate that we're calling
// run() for its side effects only; it doesn't return a value we need.
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
        case_ignore_search(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

fn main() {
    // [unwrap_or_else]:
    // If the Result is an Ok value, it returns the inner value that Ok is wrapping. However, if the value is an Err value, this method
    // calls the code in the closure.
    let config = Config::build(env::args()).unwrap_or_else(|error| {
        eprintln!("An error has occured during argument parsing:\n{error}");
        process::exit(1);
    });

    println!("Searching for pattern: '{}' in file: '{}'.", config.query, config.file_path);
    if let Err(error) = run(config) {
        eprintln!("Application Error: {error}.");
        process::exit(1);
    }
}
