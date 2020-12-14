use std::env;
use std::fs;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input_dna = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("{}", input_dna);

    let mut A_count = 0;
    let mut T_count = 0;
    let mut G_count = 0;
    let mut C_count = 0;
    let mut _X_count = 0;
    
    for nuc in input_dna.chars() {
        match nuc {
            'A' => A_count +=1,
            'T' => T_count +=1,
            'G' => G_count +=1,
            'C' => C_count +=1,
            _ => _X_count +=1,
        }
    }
    println!("A: {}, T: {}, G: {}, C: {}", A_count, T_count, G_count, C_count);
}
