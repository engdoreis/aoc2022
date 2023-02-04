use std::fs;
use regex::Regex;

fn main() {
    println!("{:?}", part_one("sample_data.txt"));
    println!("{:?}", part_one("data.txt"));
    println!("{:?}", part_two("sample_data.txt"));
    println!("{:?}", part_two("data.txt"));
}

pub fn part_one(file:&str) -> Option<u32>{
    let data = fs::read_to_string(file).unwrap();
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let parser = |num:&str| num.parse::<u32>().unwrap();
    let mut outcome = 0u32;

    for line in data.split("\n"){
        let cap = re.captures(line)?;
        outcome += (parser(&cap[1]) <= parser(&cap[3]) && parser(&cap[2]) >= parser(&cap[4]) ||
                   parser(&cap[1]) >= parser(&cap[3]) && parser(&cap[2]) <= parser(&cap[4])) as u32;
    }
    Some(outcome)
}

pub fn part_two(file:&str) -> Option<u32>{
    let data = fs::read_to_string(file).unwrap();
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let parser = |num:&str| num.parse::<u32>().unwrap();
    let mut outcome = 0u32;

    for line in data.split("\n"){
        let cap = re.captures(line)?;
        outcome += (parser(&cap[2]) >= parser(&cap[3]) && parser(&cap[1]) <= parser(&cap[4])) as u32;
    }
    Some(outcome)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_port_one(){
        assert_eq!(part_one("sample_data.txt"), Some(2));
        assert_eq!(part_one("data.txt"), Some(584));
    }
    #[test]
    fn test_port_two(){
        assert_eq!(part_two("sample_data.txt"), Some(4));
        assert_eq!(part_two("data.txt"), Some(933));
    }
}