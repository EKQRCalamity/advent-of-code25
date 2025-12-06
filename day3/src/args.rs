use std::{env, fmt::Display, path::PathBuf};

pub struct ArgParser {
    pub file: PathBuf,
}

impl Display for ArgParser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ArgParser {{ file: {} }}",
            self.file.to_string_lossy()
        )
    }
}

impl ArgParser {
    pub fn new() -> Self {
        let mut args = env::args_os();

        args.next();

        let file_arg = args.next().expect("No argument supplied!");

        let path = PathBuf::from(&file_arg);
        if !path.exists() {
            panic!("Argument is not a valid file path: '{}'",
                path.to_string_lossy());
        }

        Self { file: path }
    }
}
