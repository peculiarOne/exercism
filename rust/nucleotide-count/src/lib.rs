use std::collections::HashMap;

const VALID_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !VALID_NUCLEOTIDES.contains(&nucleotide) {
        Err(nucleotide)
    } else {
        Ok(get_single_nucleotide_count(
            &get_valid_nucleotides(dna)?,
            nucleotide,
        ))
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let good_dna = get_valid_nucleotides(dna)?;
    let mut counts = HashMap::new();
    for nucleotide in &VALID_NUCLEOTIDES {
        counts.insert(
            *nucleotide,
            get_single_nucleotide_count(&good_dna, *nucleotide),
        );
    }
    Ok(counts)
}

fn get_valid_nucleotides(dna: &str) -> Result<&str, char> {
    for c in dna.chars() {
        if !VALID_NUCLEOTIDES.contains(&c) {
            return Err(c);
        }
    }
    Ok(dna)
}

fn get_single_nucleotide_count(good_dna: &str, nucleotide: char) -> usize {
    good_dna.chars().filter(|&c| c == nucleotide).count()
}
