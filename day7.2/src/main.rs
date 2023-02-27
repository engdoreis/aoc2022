use regex::Regex;
use std::fs;
use std::slice::IterMut;
use std::iter::Peekable;

fn main() {
    // println!("Part_one: {}", part_one("sample_data.txt").unwrap());
    println!("Part_one: {}", part_one("data.txt").unwrap());
}

fn part_one(file: &str) -> Option<String> {
    let data = fs::read_to_string(file).unwrap();
    let mut paths = Vec::new();
    for line in data.split("\n") {
        // println!("{}", line);
        if let Some(res) = Path::from_str(line){
            paths.push(res);
        }
    }
    // println!("{:#?}", paths);
    let mut iter_paths = paths.iter_mut().peekable();
    Path::compute_sizes(&mut iter_paths);
    // println!("{:#?}", paths);
    // let query = Path::query_by(&paths, |path| path.types == PathType::Dir && path.size <= 100_000)?
    // .iter().map(|x| x.name.clone()).collect::<String>();
    // println!("{:#?}", query);


    let query = paths.iter()
    .filter(|path| path.types == PathType::Dir && path.size <= 100_000)
    .map(|x| x.name.clone()).collect::<String>();
    Some(query)
}

#[derive(Debug, PartialEq)]
enum PathType {
    File,
    Dir,
}
#[derive(Debug)]
struct Path {
    name: String,
    level: u32,
    types: PathType,
    size: u32,
}

impl Path {
    fn from_str(text: &str) -> Option<Self> {
        let re = Regex::new(r#"(?P<level>\s*)-\s(?P<name>[\w./]+)\s\((?P<type>dir|file)(:?,\ssize=)?(?P<size>\d+)?\)"#).unwrap();
        let caps = re.captures(text)?;

        Some(Path {
            level: (caps["level"].len() / 2) as u32,
            name : caps["name"].to_string(),
            types: match &caps["type"] {
                "dir" => PathType::Dir,
                "file" => PathType::File,
                _ => panic!(),
            },
            size : match &caps["type"] {
                "dir" => 0,
                "file" => caps["size"].parse::<u32>().unwrap(),
                _ => panic!(),
            }
        })
    }

    fn compute_sizes(paths: &mut Peekable<IterMut<Path>>) -> u32{
        let mut total = 0u32;
        let mut cur_level = 0u32;

        loop{
            if let Some(path) = paths.next_if(|path| path.level >= cur_level){
                match &path.types {
                    PathType::File => {
                        total += path.size;
                    }
                    PathType::Dir => {
                        let dir_size = Self::compute_sizes(paths);
                        total += dir_size;
                        path.size =  dir_size;
                    }
                }
                cur_level = path.level;
                continue;
            }
            break;
        }
        total
    }

    fn query_by<'a>(paths: &'a Vec<Self>, by: fn(&Self)->bool) -> Option<Vec<&'a Self>>{
        let mut outcome = Vec::new();
        for path in  paths.iter(){
            if by(path){
                outcome.push(path);
            }
        }
        if outcome.is_empty(){
            return None;
        }
        Some(outcome)
    }
}


#[cfg(test)]
mod unittest {
    use super::*;
    #[test]
    fn test_part_one() {
        // assert_eq!(part_one("sample_data.txt"), Some(String::from("ae")));
        assert_eq!(part_one("data.txt"), Some(String::from("ab")));
    }
}
