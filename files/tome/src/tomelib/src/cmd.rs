use std::process::{Child, Command, ExitStatus};
use std::ffi::OsStr;
use std::process::Output;


/// Runs command/program with or without parameters
pub fn cmd<S, I>(cmd: &'_ str, args: Option<I>) -> Result<ExitStatus, std::io::Error>
where I: IntoIterator<Item = S>, S: AsRef<OsStr> {
    let mut cmd = match args {
        Some(args) => Command::new(cmd).args(args).spawn()?,
        None => Command::new(cmd).spawn()?,
    };

    Ok(cmd.wait()?)
}

/// Runs the command and tries to suppress its output
/// - may not work with cargo and other specific commands
pub fn cmd_with_output<S, I>(cmd: &'_ str, args: Option<I>) -> Result<Output, std::io::Error>
where I: IntoIterator<Item = S>, S: AsRef<OsStr> {

    match args {
        Some(args) => Command::new(cmd).args(args).output(),
        None => Command::new(cmd).output()
    }

}

/// runs command and leaves its handle
pub fn cmd_async<S, I>(cmd: &'_ str, args: Option<I>) -> Result<Child, std::io::Error>
where I: IntoIterator<Item = S>, S: AsRef<OsStr> {
    Ok(match args {
        Some(args) => Command::new(cmd).args(args).spawn()?,
        None => Command::new(cmd).spawn()?,
    })
}


/// Tries to search for a command in `PATH`
pub fn search(command: &str) -> Result<String, ()> {

    let path = std::env::var("PATH").map_err(|_| ())?;

    let paths = path.split(':');

    for i in paths {
        let files = match std::fs::read_dir(i) {
            Ok(f) => f,
            Err(_) => return Err(()),
        };

        for file in files {
            if let Ok(f) = file {
                let name = f.file_name();
                if name == command {
                    //return Ok(name.into_string().map_err(|_| ())?)
                    return Ok(f.path().to_str().unwrap().to_string());
                }
            }
        }
    }

    Err(())

}

#[macro_export]
macro_rules! cmd {
    ($cmd:expr) => {
        use crate::cmd::cmd;
        cmd($cmd, None)
    };
    ($cmd:expr, $($arg:expr),* $(,)?) => {{
        use crate::cmd::cmd;
        cmd($cmd, Some(&[$($arg),*]))
    }};
}