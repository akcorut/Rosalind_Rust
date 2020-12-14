use std::env;
use std::fs;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input_dna = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("DNA: {}", input_dna);
    let mut rev_comp = String::new();

    for nuc in input_dna.trim().chars().rev() {
        match nuc {
            'A' => rev_comp.push('T'),
            'T' => rev_comp.push('A'),
            'G' => rev_comp.push('C'),
            'C' => rev_comp.push('G'),
            _ => rev_comp.push('n'),
        }
    }
    println!("The reverse complement: {}", rev_comp)
}
