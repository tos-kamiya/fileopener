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