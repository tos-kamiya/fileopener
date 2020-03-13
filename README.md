fileopener
==========

Functions to open/create a file if file name is given, return the standard input/output otherwise.

## Install/Usage

Put code to some directory.

Add the following lines to `Cargo.toml`:

```
[dependencies]
fileopener = { path = "path/to/fileopener" }
```

Use functions like:

```rust
use std::io::{BufRead, Write};

fn main() {
    // let input_file = "input.txt"; 
    let input_file = "-"; // read from the standard input
    let output_file = "output.txt";
    // let output_file = "-"; // write to the standard output
    
    let outp = fileopener::create_file(output_file).unwrap();
    let mut writer = std::io::BufWriter::new(outp);
    
    let inp = fileopener::open_file(input_file).unwrap();
    let reader = std::io::BufReader::new(inp);
    for line in reader.lines() {
        let line = line.unwrap();
        writeln!(writer, "{}", line).unwrap();
    }
    
    writer.flush().unwrap();    
}
```
