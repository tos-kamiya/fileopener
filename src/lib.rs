use std::fs;
use std::io;
use std::os::unix::io::{FromRawFd, AsRawFd};

pub fn open_file(input_file: &str) -> io::Result<fs::File> {
    match input_file {
        "" | "-" => Ok(unsafe { fs::File::from_raw_fd(io::stdin().as_raw_fd()) }),
        _ => Ok(fs::File::open(input_file)?)
    }
}

pub fn create_file(output_file: &str) -> io::Result<fs::File> {
    match output_file {
        "" | "-" => Ok(unsafe { fs::File::from_raw_fd(io::stdout().as_raw_fd()) }),
        _ => Ok(fs::File::create(output_file)?),
    }
}

