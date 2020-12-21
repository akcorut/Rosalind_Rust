use std::env;
use std::fs;
use std::time::{Duration, Instant};

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let input_dna = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("DNA: {}", input_dna);

    let mut output_rna = String::new();

    let start = Instant::now();
    for nuc in input_dna.chars() {
        match nuc {
            'T' => output_rna.push('U'),
            't' => output_rna.push('u'),
            _ => output_rna.push(nuc),
        }
    }
    println!("RNA: {}", output_rna);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
