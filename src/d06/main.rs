use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn num_unique(s: &str) -> usize {
    let mut hist: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        hist.insert(c, *hist.get(&c).unwrap_or(&0) + 1);
    }
    println!("{:?}", hist);
    return hist.iter().filter(|(_, &n)| n == 1).count();
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("src/d06/input")?;
    let window_size = 14;
    let result = (window_size..content.len()).find_map(|end| {
        let start = end - window_size;
        if num_unique(&content[start..end]) == window_size {
            return Some(end);
        }
        return None;
    });
    println!("{:?}", result);

    Ok(())
}
