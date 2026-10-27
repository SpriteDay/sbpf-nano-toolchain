use std::{env, fmt, fs, io};

/// Expands argfiles in command line arguments.
#[derive(Default)]
struct Expander {
    shell_argfiles: bool,
    expanded: Vec<String>,
}

impl Expander {
    /// Handles the next argument. If the argument is an argfile, it is expanded
    fn arg(&mut self, arg: &str) -> Result<(), Error> {
        if let Some(argfile) = arg.strip_prefix('@') {
            match argfile.split_once(':') {
                Some(("shell", path)) if self.shell_argfiles => {
                    shlex::split(&Self::read_file(path)?)
                        .ok_or_else(|| Error::ShellParserError(path.to_string()))?
                        .into_iter()
                        .for_each(|arg| self.expanded.push(arg));
                }
                _ => {
                    let contents = Self::read_file(argfile)?;
                    contents
                        .lines()
                        .for_each(|arg| self.expanded.push(arg.to_string()));
                }
            }
        } else {
            self.expanded.push(arg.to_string());
        }

        Ok(())
    }

    /// Consumes the `Expander`, returning the expanded arguments
    fn finish(self) -> Vec<String> {
        self.expanded
    }

    /// Reads the contents of a file as UTF-8.
    fn read_file(path: &str) -> Result<String, Error> {
        fs::read_to_string(path).map_err(|e| {
            if e.kind() == io::ErrorKind::InvalidData {
                Error::Utf8Error(path.to_string())
            } else {
                Error::IOError(path.to_string(), e)
            }
        })
    }
}

/// Gets the raw unprocessed command-line arguments as Unicode strings, without doing any further
/// processing (e,g, without `@file` expansion)
pub fn raw_args() -> Vec<String> {
    let mut args = Vec::new();
    let mut guar = Ok(());
    for (i, arg) in env::args_os().enumerate() {
        match arg.into_string() {
            Ok(arg) => args.push(arg),
            Err(arg) => {
                guar = Err(format!("Argument {i} is not valid Unicode: {arg:?}"));
            }
        };
    }
    if let Err(guar) = guar {
        panic!("{guar}");
    }
    args
}

/// Replaces any `@file` arguments with the contents of `file`, with each line of `file` as a
/// separate argument
///
/// **Note:*8 This function doesn't interpret argument 0 in any special way.
/// If this function is intended to be used with command line arguments,
/// `argv[0]` must be removed prior to calling it manually.
pub fn arg_expand_all(at_args: &[String]) -> Vec<String> {
    let mut expander = Expander::default();
    let mut result = Ok(());
    for arg in at_args {
        if let Err(err) = expander.arg(arg) {
            result = Err(format!("Failed to load argument file: {err}"));
        }
    }
    if let Err(guar) = result {
        panic!("{guar}");
    }
    expander.finish()
}

#[derive(Debug)]
enum Error {
    Utf8Error(String),
    IOError(String, io::Error),
    ShellParserError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Utf8Error(path) => write!(fmt, "UTF-8 error in {path}"),
            Error::IOError(path, err) => write!(fmt, "IO error: {path}: {err}"),
            Error::ShellParserError(path) => {
                write!(fmt, "Invalid shell-style arguments in: {path}")
            }
        }
    }
}
