pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

// not terribly efficient, but I wanted to see how calculating using the
// previous row could work
impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let rows = if row_count == 0 {
            vec![]
        } else {
            let mut rows: Vec<Vec<u32>> = vec![vec![1]];
            for i in 1..row_count as usize {
                rows.push(next_row(&rows[i - 1]));
            }
            rows
        };
        PascalsTriangle { rows: rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}

fn next_row(row: &Vec<u32>) -> Vec<u32> {
    let mut i_row = vec![0];
    i_row.extend(row.iter());
    i_row.push(0);

    i_row[1..]
        .iter()
        .zip(i_row[..i_row.len()].iter())
        .map(|(a, b)| a + b)
        .collect()
}
