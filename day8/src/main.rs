// use regex::Regex;
use std::fs;

fn main() {
    println!("Part_one: {}", part_one("sample_data.txt").unwrap());
    println!("Part_one: {}", part_one("data.txt").unwrap());
    println!("Part_one: {}", part_two("sample_data.txt").unwrap());
    // println!("Part_one: {}", part_two("data.txt").unwrap());
}

fn part_one(file: &str) -> Option<String> {
    let data = fs::read_to_string(file).unwrap();
    let mut florest = Florest::from_string(&data);
    florest.discover_visible();
    println!("{florest}");
    let visible_trees = florest.query_tree_by(|tree| tree.visible).len();
    Some(visible_trees.to_string())
}

fn part_two(file: &str) -> Option<u32> {
    let data = fs::read_to_string(file).unwrap();
    let mut florest = Florest::from_string(&data);
    florest.discover_visible();
    florest.compute_view_score();
    println!("{florest}");
    florest.query_tree_by(|tree| tree.scenic_score > 0).iter().map(|x| x.scenic_score).max()
}

#[derive(Debug, Clone)]
struct Tree {
    size: u32,
    visible: bool,
    scenic_score: u32,
}
impl Tree {
    fn new(size: u32) -> Self {
        Tree {
            size,
            visible: false,
            scenic_score: 0,
        }
    }
}
#[derive(Debug)]
struct Florest {
    trees: Vec<Vec<Tree>>,
}

impl Florest {
    fn from_string(text: &str) -> Self {
        let mut trees = Vec::new();
        for line in text.split("\n") {
            trees.push(
                line.chars()
                    .map(|c| Tree::new(c.to_digit(10).unwrap()))
                    .collect::<Vec<_>>(),
            );
        }
        Florest { trees }
    }
    fn compute_view_score(&mut self) {
        for y in 0..self.trees.len() {
            for x in 0..self.trees[0].len() {
                // Look forward
                self.trees[y][x].scenic_score = self.compute_visible_trees((y, x), |(y, x)| (y, x+1) ) *
                // Look backward
                self.compute_visible_trees((y, x), |(y, x)| (y, x-1) ) *
                // Look Down
                self.compute_visible_trees((y, x), |(y, x)| (y+1, x) ) *
                // Look Up
                self.compute_visible_trees((y, x), |(y, x)| (y-1, x) );
            }
        }
    }
    fn compute_visible_trees<F>(&mut self, origin: (usize, usize), increment: F) -> u32
    where
        F: Fn((i32, i32)) -> (i32, i32),
    {
        let mut visible_trees = 0u32;
        let mut maximum_size = -1i32;
        let current_size = self.trees[origin.0][origin.1].size;
        let mut cursor = (origin.0 as i32, origin.1 as i32);
        'outer: loop {
            cursor = increment(cursor);
            let size = match self.trees.get(cursor.0 as usize) {
                Some(row) => match row.get(cursor.1 as usize) {
                    Some(tree) => tree.size as i32,
                    None => break 'outer,
                },
                None => break 'outer,
            };
            if size > maximum_size || size < current_size as i32{
                visible_trees += 1;
                maximum_size = size;
            }
            if size >= current_size as i32{
                break;
            }
        }
        std::cmp::max(0, visible_trees)
    }

    fn discover_visible(&mut self) {
        self.search_horizontally();
        self.search_vertically();
    }

    fn search_vertically(&mut self) {
        self.search_up_down();
        self.search_down_up();
    }

    fn search_down_up(&mut self) {
        for x in 0..self.trees[0].len() {
            let mut tallest_tree = -1i32;
            for y in (0..self.trees.len()).rev() {
                if self.trees[y][x].size as i32 > tallest_tree {
                    self.trees[y][x].visible = true;
                    tallest_tree = self.trees[y][x].size as i32;
                }
            }
        }
    }

    fn search_up_down(&mut self) {
        // Search Up-Down.
        for x in 0..self.trees[0].len() {
            let mut tallest_tree = -1i32;
            for y in 0..self.trees.len() {
                //Up-Down.
                if self.trees[y][x].size as i32 > tallest_tree {
                    self.trees[y][x].visible = true;
                    tallest_tree = self.trees[y][x].size as i32;
                }
            }
        }
    }

    fn search_horizontally(&mut self) {
        for row in self.trees.iter_mut() {
            let mut tallest_tree = -1i32;
            //Check forward.
            for tree in row.iter_mut() {
                // Check forward and up-down.
                if tree.size as i32 > tallest_tree {
                    tree.visible = true;
                    tallest_tree = tree.size as i32;
                }
            }

            // Check backward.
            tallest_tree = -1;
            for tree in row.iter_mut().rev() {
                if tree.size as i32 > tallest_tree {
                    tree.visible = true;
                    tallest_tree = tree.size as i32;
                }
            }
        }
    }

    fn query_tree_by<T>(&self, by: T) -> Vec<&Tree>
    where
        T: Fn(&Tree) -> bool,
    {
        let mut outcome = Vec::new();
        for row in self.trees.iter() {
            for tree in row.iter() {
                if by(tree) {
                    outcome.push(tree);
                }
            }
        }
        outcome
    }
}

impl std::fmt::Display for Florest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.trees.iter() {
            for tree in row.iter() {
                write!(
                    f,
                    "{}{}{} ",
                    tree.size,
                    tree.scenic_score,
                    if tree.visible { "v" } else { "i" }
                )?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

#[cfg(test)]
mod unittest {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one("sample_data.txt"), Some("21".to_string()));
        assert_eq!(part_one("data.txt"), Some("1688".to_string()));
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two("sample_data.txt"), Some(8));
        assert_eq!(part_two("data.txt"), Some(410400));
    }
}
