use std::fs;

fn main() {
    println!("sum {:?}", part_one());
    println!("sum {:?}", part_two());
}


pub fn part_one() -> u32{
    let data = fs::read_to_string("data.txt").unwrap();

    let mut sum = 0u32;
    for line in data.split("\n").collect::<Vec<_>>().iter(){
        // let char = 'A';
        let len = line.len();
        let chunks = line.chars().collect::<Vec<char>>()
        .chunks(len/2)
        .map(|c| c.to_vec())
        .collect::<Vec<_>>();

        let c = find_duplicates(chunks);
        let p:u32 = c.iter().map(|x| map_char_to_priority(*x)).sum();
        sum += p;
    }
    sum
}


pub fn part_two() -> u32{

    let data = fs::read_to_string("data.txt").unwrap();

    let mut sum = 0u32;
    for chunks in data.split("\n").collect::<Vec<_>>().windows(3).step_by(3){
        let chunks = chunks.to_vec()
        .into_iter()
        .map(|c| c.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<char>>>();
        let c = find_duplicates(chunks);
        // println!("{:?}", c);
        let p:u32 = c.iter().map(|x| map_char_to_priority(*x)).sum();
        sum += p;
    }
    sum
}

fn map_char_to_priority(c:char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 96,
        'A'..='Z' => c as u32 - 38,
        _ => 0
    }
}

fn find_duplicates(compartments:Vec<Vec<char>>) -> Vec<char> {
    // println!("{} {}", compartment_1, compartment_2);
    let mut result = Vec::<char>::new();
    let first_chunk = compartments.first().unwrap();
    'outer: for c in first_chunk.iter(){
        for comp in compartments.iter().skip(1){
            if ! comp.contains(c){
                continue 'outer;
            }
        }
        if !result.contains(c){
            result.push(*c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one(){
        assert_eq!(part_one(), 7597);
    }
    #[test]
    fn test_part_two(){
        assert_eq!(part_two(), 2607);
    }
}