use std::fs;
use std::cmp;
use std::fmt;
use regex::Regex;

fn main() {
    println!("Part_one: {}", part_one("sample_data.txt").unwrap());
    println!("Part_one: {}", part_one("data.txt").unwrap());

    println!("Part_two: {}", part_two("sample_data.txt").unwrap());
    println!("Part_two: {}", part_two("data.txt").unwrap());
}

pub fn part_one(file: &str) -> Option<String>{
    let data = fs::read_to_string(file).unwrap();

    let (crates, instructions) = data.split_once("\n\n").unwrap();
    let mut crates = CratesStack::from_string(crates)?;
    println!("{}", crates);
    let crane = CrateMover9000::new();

    for instruction in instructions.split("\n"){
        crane.process_instruction(&mut crates, instruction)?;
        println!("{}", crates);
    }

    crates.top()
}

pub fn part_two(file: &str) -> Option<String>{
    let data = fs::read_to_string(file).unwrap();

    let (crates, instructions) = data.split_once("\n\n").unwrap();
    let mut crates = CratesStack::from_string(crates)?;
    println!("{}", crates);
    let crane = CrateMover9001::new();

    for instruction in instructions.split("\n"){
        crane.process_instruction(&mut crates, instruction)?;
        println!("{}", crates);
    }

    crates.top()
}

#[derive(Debug)]
struct CratesStack{
    stacks: Vec<Vec<String>>,
    stack_re:Regex,
}

impl fmt::Display for CratesStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for stack in self.stacks.iter(){
            output += &(stack.join(" ") + "\n");
            
        }
        write!(f, "{}", output)
    }
}

impl CratesStack{
    fn new()-> Self{
        CratesStack{
            stacks : Vec::new(),
            stack_re: Regex::new(r"\[(\w)\]").unwrap(),
        }
    }

    fn from_string(input:&str) -> Option<Self>{
        let mut crates = Self::new();
        let lines = input.split("\n").collect::<Vec<_>>();
        let lines_iter = lines.into_iter();
        let num_of_stacks = lines_iter.clone()
                                    .last()?
                                    .split_whitespace().last()?
                                    .parse().ok()?;
        for mut line in lines_iter.rev().skip(1){
            for i in 0..num_of_stacks{
                if crates.stacks.len() <= i{
                    crates.stacks.push(Vec::new());
                }
                let (next, reminder) = line.split_at(cmp::min(4, line.len()));
                if let Some(cap) = crates.stack_re.captures(next){
                    crates.stacks[i].push(String::from(&cap[1]));
                }
                line = reminder;
            }
        }
        Some(crates)
    }
    
    fn top(self) -> Option<String>{
        Some( self.stacks.into_iter()
                        .map(|stack| stack.into_iter().last().unwrap())
                        .collect::<Vec<_>>()
                        .join(" ")
        )
    }

}

struct CrateMover9000{
    instruction_re:Regex,
}

impl CrateMover9000{
    fn new()-> Self{
        CrateMover9000{
            instruction_re: Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap(),
        }
    }

    fn process_instruction(&self, crates_stack:&mut CratesStack, instruction: &str) -> Option<()>{
        let (moves, from, to) = self.parse_command(instruction)?;

        let size = crates_stack.stacks[from].len();
        let mut to_move = crates_stack.stacks[from].split_off(size - moves);

        to_move = to_move.into_iter().rev().collect::<Vec<_>>();
        crates_stack.stacks[to].append(&mut to_move);
        Some(())
    }

    fn parse_command(&self, instruction: &str) -> Option<(usize, usize, usize)> {
        let cap = self.instruction_re.captures(instruction)?;
        let moves:usize = cap[1].parse().unwrap();
        let from = cap[2].parse::<usize>().unwrap() - 1;
        let to = cap[3].parse::<usize>().unwrap() - 1;
        Some((moves, from, to))
    }
}

struct CrateMover9001{
    parent: CrateMover9000,
}

impl CrateMover9001{
    fn new()-> Self{
        CrateMover9001{
            parent: CrateMover9000::new(),
        }
    }

    fn process_instruction(&self, crates_stack:&mut CratesStack, instruction: &str) -> Option<()>{
        let (moves, from, to) = self.parent.parse_command(instruction)?;

        let size = crates_stack.stacks[from].len();
        let mut to_move = crates_stack.stacks[from].split_off(size - moves);
        crates_stack.stacks[to].append(&mut to_move);
        Some(())
    }
}

#[cfg(test)]
mod unittest{
    use super::*;
    #[test]
    fn test_part_one(){
        assert_eq!(part_one("sample_data.txt"), Some(String::from("C M Z")));
        assert_eq!(part_one("data.txt"), Some(String::from("T W S G Q H N H L")));
    }

    #[test]
    fn test_part_two(){
        assert_eq!(part_two("sample_data.txt"), Some(String::from("M C D")));
        assert_eq!(part_two("data.txt"), Some(String::from("J N R S C D W P P")));
    }
}