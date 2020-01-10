use std::collections::HashMap;

const VALID_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !VALID_NUCLEOTIDES.contains(&nucleotide) {
        Err(nucleotide)
    } else {
        match get_valid_nucleotides(dna) {
            Ok(good_dna) => Ok(get_single_nucleotide_count(&good_dna, nucleotide)),
            Err(bad) => Err(bad),
        }
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    match get_valid_nucleotides(dna) {
        Ok(good_dna) => {
            let mut counts = HashMap::new();
            for nucleotide in &VALID_NUCLEOTIDES {
                counts.insert(
                    *nucleotide,
                    get_single_nucleotide_count(&good_dna, *nucleotide),
                );
            }
            Ok(counts)
        }
        Err(bad) => Err(bad),
    }
}

fn get_valid_nucleotides(dna: &str) -> Result<Vec<char>, char> {
    let (valid, invalid): (Vec<char>, Vec<char>) =
        dna.chars().partition(|c| VALID_NUCLEOTIDES.contains(c));

    if !invalid.is_empty() {
        Err(invalid[0])
    } else {
        Ok(valid)
    }
}

fn get_single_nucleotide_count(good_dna: &[char], nucleotide: char) -> usize {
    good_dna.iter().filter(|&c| c == &nucleotide).count()
}
