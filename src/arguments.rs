use getopts::Options;
use std::path::{PathBuf};

fn get_options() -> Options {
    let mut options = Options::new();
    options.optflag(
        "h",
        "help",
        "Displays this message"
    );
    options.optflagopt(
        "c",
        "config",
        "The configuration file to use",
        "CONFIG"
    );
    options.optflagopt(
        "i",
        "input",
        "A file containing binary protobuf messages",
        "INPUT"
    );

    options
}

pub struct Arguments {
    pub config: PathBuf,
    pub input: PathBuf,
}

impl Arguments {
    pub fn new(config_path: &str, input_path: &str) -> Self {
        Self {
            config: PathBuf::from(config_path),
            input: PathBuf::from(input_path),
        }
    }

    pub fn parse(args: &[String]) -> Result<Arguments, String> {
        let program_name = args[0].clone();
        let options = get_options();

        let matches = match options.parse(&args[1..]) {
            Ok(m) => { m }
            Err(f) => { panic!(f.to_string()); }
        };

        // If the flag is present, print usage details.
        if matches.opt_present("h") {
            let short_usage = options.short_usage(&program_name);
            println!("{}", options.usage(&short_usage));
            return Err(String::new());
        }

        // Parse config argument.
        let config = match matches.opt_str("c") {
            Some(s) => {
                if s.is_empty() {
                    // Option is present, but without any argument.
                    return Err("Config option is present, but without any argument!".to_string());
                }

                s
            }
            None => { "".to_string() }
        };

        // Parse input argument.
        let input = match matches.opt_str("i") {
            Some(s) => {
                if s.is_empty() {
                    // Option is present, but without any argument.
                    return Err(
                        "Please provide an input file to read protobuf data from!".to_string()
                    );
                }

                s
            }
            None => {
                return Err("Please provide an input file to read protobuf data from!".to_string());
            }
        };

        // Return Arguments struct that holds PathBufs to the config and input file.
        Ok(Arguments::new(config.as_str(), input.as_str()))
    }
}

#[cfg(test)]
mod test {
    use crate::arguments::Arguments;

    #[test]
    fn test_parse_args() {
        let file_name = "test_file";
        let args: Vec<String> = vec![String::from("test"), String::from("-i"), file_name];

        let arguments = match Arguments::parse(&args) {
            Ok(a) => { assert_eq!(arguments.input.to_str().unwrap(), file_name) }
            Err(f) => {
                // Any empty error here is considered an early bail.
                if f.is_empty() {
                    return;
                }

                panic!(f.to_string())
            }
        };
    }
}
