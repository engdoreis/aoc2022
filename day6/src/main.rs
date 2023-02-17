use std::fs;
use std::collections::HashSet;

fn main() {
    println!("Part_one: {}", part_one("sample_data.txt").unwrap());
    println!("Part_one: {}", part_one("data.txt").unwrap());

    println!("Part_two: {}", part_two("sample_data.txt").unwrap());
    println!("Part_two: {}", part_two("data.txt").unwrap());
}


pub fn part_one(file: &str) -> Option<u32>{
    let data = fs::read_to_string(file).unwrap();
    
    Some(parse(&data, 4) as u32)
}

pub fn part_two(file: &str) -> Option<u32>{
    let data = fs::read_to_string(file).unwrap();
    Some(parse(&data, 14)  as u32)

}

fn parse(data:&str, windows_size:usize) -> usize {
    for (pos,slice) in data.chars().collect::<Vec<char>>().windows(windows_size).enumerate(){
        // Check that theres not duplicated values.
        let mut uniq = HashSet::new();
        slice.iter().all(|x| uniq.insert(x));
        if uniq.len() == slice.len(){
            return pos + slice.len();
        }   
    }
    0
}

#[cfg(test)]
mod unittest{
    use super::*;
    #[test]
    fn test_part_one(){
        assert_eq!(part_one("sample_data.txt"), Some(11));
        assert_eq!(part_one("data.txt"), Some(1707));
    }

    #[test]
    fn test_part_two(){
        assert_eq!(part_two("sample_data.txt"), Some(26));
        assert_eq!(part_two("data.txt"), Some(3697));
    }
}