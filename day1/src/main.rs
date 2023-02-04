use std::fs;

fn main() {
    part_one();
    part_two();
}

#[derive(Debug, Copy, Clone)]
struct ElfSupply{
    index:usize,
    calories:u32
}

impl ElfSupply {
    fn new(i:usize,c:u32) -> Self{
        ElfSupply {index:i, calories:c}
    }
}

fn part_one(){
    let data =fs::read_to_string("data.txt").unwrap();
    let mut wealthier_elf = ElfSupply {index:0, calories:0};
    
    for (i, elf) in data.split("\n\n").collect::<Vec<&str>>().iter().enumerate(){
        let calories:u32 = elf.split("\n").collect::<Vec<&str>>().iter().map(|x| x.parse::<u32>().unwrap()).sum();
        if wealthier_elf.calories<calories{
            wealthier_elf.calories = calories;
            wealthier_elf.index = i + 1;
        }
    }
    println!("The Elf {0} is the wealthier whith {1} calories.", wealthier_elf.index, wealthier_elf.calories);
}

fn part_two(){
    let data =fs::read_to_string("data.txt").unwrap();
    let mut top_wealthier_elv= vec![ElfSupply::new(0,0);3];

    for (i, elf) in data.split("\n\n").collect::<Vec<&str>>().iter().enumerate(){
        let calories:u32 = elf.split("\n").collect::<Vec<&str>>().iter().map(|x| x.parse::<u32>().unwrap()).sum();
        for (index, wealth_elf) in top_wealthier_elv.clone().iter().enumerate(){
            if wealth_elf.calories < calories {
                top_wealthier_elv.insert(index, ElfSupply::new(i + 1, calories));
                top_wealthier_elv.pop();
                break;
            }
        }
    }

    println!("Top wealthier elv:");
    for elf in top_wealthier_elv.iter(){
        println!(" {:?}: {:?} calories", elf.index, elf.calories);
    }
    println!("Total: {:?} calories", top_wealthier_elv.iter().map(|x| x.calories).sum::<u32>());
}
