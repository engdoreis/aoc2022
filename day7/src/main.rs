// use regex::Regex;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
// use std::iter::Peekable;
// use std::slice::{Iter, IterMut};
use std::str::Split;

fn main() {
    println!("Part_one: {}", part_one("sample_data.txt").unwrap());
    println!("Part_one: {}", part_one("data.txt").unwrap());
    println!("Part_one: {}", part_two("sample_data.txt").unwrap());
    println!("Part_one: {}", part_two("data.txt").unwrap());
}

fn part_one(file: &str) -> Option<usize> {
    let data = fs::read_to_string(file).unwrap();
    let mut state_machine = StateMachine::new(&data);
    while let Some(num) = state_machine.process() {}
    state_machine.tree.compute_sizes();
    match state_machine
        .tree
        .query_by(|node| node.is_dir && node.size < 100_000)
    {
        Some(list) => Some(list.iter().fold(0, |sum, node| sum + node.size)),
        None => None,
    }
}

fn part_two(file: &str) -> Option<usize> {
    let disk_needed = 30_000_000usize;
    let data = fs::read_to_string(file).unwrap();
    let mut state_machine = StateMachine::new(&data);
    while let Some(num) = state_machine.process() {}
    let disk_used = state_machine.tree.compute_sizes();
    let disk_to_free = disk_needed - (70_000_000 - disk_used);
    let dir = match state_machine
        .tree
        .query_by(|node| node.is_dir && node.size >= disk_to_free)
    {
        Some(list) => list.into_iter().min_by_key(|x| x.size),
        None => None,
    };
    Some(dir.unwrap().size)
}
#[derive(Debug, Clone, Default)]
struct NodeInfo<'a> {
    size: usize,
    name: &'a str,
    is_dir: bool,
}

impl<'a> NodeInfo<'a> {
    fn new(name: &'a str, is_dir: bool, size: usize) -> Self {
        NodeInfo { name, is_dir, size }
    }
}

#[derive(Debug, Clone)]
enum FsNode<'a> {
    File(RefCell<NodeInfo<'a>>, Option<Rc<FsNode<'a>>>),
    Dir(
        RefCell<NodeInfo<'a>>,
        Option<Rc<FsNode<'a>>>,
        RefCell<Vec<Rc<FsNode<'a>>>>,
    ),
}

#[derive(Debug, Clone)]
struct FsTree<'a> {
    root: Rc<FsNode<'a>>,
    cursor: Rc<FsNode<'a>>,
}

impl<'a> FsTree<'a> {
    pub fn new() -> Self {
        let root = Rc::new(FsNode::Dir(
            RefCell::new(NodeInfo::default()),
            None,
            RefCell::new(Vec::new()),
        ));
        FsTree {
            cursor: Rc::clone(&root),
            root,
        }
    }
    pub fn from_str(name: &'a str) -> Self {
        let root = Rc::new(FsNode::Dir(
            RefCell::new(NodeInfo::new(name, true, 0)),
            None,
            RefCell::new(Vec::new()),
        ));
        FsTree {
            cursor: Rc::clone(&root),
            root,
        }
    }
    fn change_dir(&mut self, name: &str) -> u32 {
        self.cursor = match (name, &*self.cursor) {
            ("..", FsNode::Dir(_, Some(parent), _)) => Rc::clone(&parent),
            (name, FsNode::Dir(_, _, children)) => {
                if let Some(fit) = children
                    .borrow()
                    .iter()
                    .find(|candidate| match &***candidate {
                        FsNode::Dir(info, _, _) => info.borrow().name.eq(name),
                        _ => false,
                    })
                {
                    Rc::clone(fit)
                } else {
                    return 0;
                }
            }
            (_, _) => todo!(),
        };
        2
    }
    fn add_dir(&mut self, name: &'a str) -> u32 {
        if let FsNode::Dir(info, _, children) = &*self.cursor {
            let new_info = NodeInfo::new(name, true, 0);
            children.borrow_mut().push(Rc::new(FsNode::Dir(
                RefCell::new(new_info),
                Some(Rc::clone(&self.cursor)),
                RefCell::new(Vec::new()),
            )));
        }
        2
    }
    fn add_file(&mut self, name: &'a str, size: usize) -> u32 {
        if let FsNode::Dir(info, _, children) = &*self.cursor {
            let new_info = NodeInfo::new(name, false, size);
            children.borrow_mut().push(Rc::new(FsNode::File(
                RefCell::new(new_info),
                Some(Rc::clone(&self.cursor)),
            )));
        }
        2
    }
    pub fn compute_sizes(&self) -> usize {
        if let FsNode::Dir(info, _, children) = &*self.root {
            let size = FsTree::compute_dir_size(&*children.borrow());
            let mut info = info.borrow_mut();
            info.size = size;
            return size;
        }
        0
    }
    fn compute_dir_size(files: &Vec<Rc<FsNode>>) -> usize {
        files
            .iter()
            .map(|file| match &**file {
                FsNode::File(info, _) => info.borrow().size,
                FsNode::Dir(info, _, children) => {
                    let mut info = info.borrow_mut();
                    let size = FsTree::compute_dir_size(&*children.borrow());
                    info.size = size;
                    size
                }
                _ => unreachable!(),
            })
            .sum()
    }

    fn query_by_recursive(
        &mut self,
        by: &dyn Fn(&NodeInfo) -> bool,
        outcome: &mut Vec<NodeInfo<'a>>,
    ) {
        let cursor = Rc::clone(&self.cursor);
        match &*cursor {
            FsNode::Dir(info, _, children) => {
                if by(&info.borrow()) {
                    outcome.push(info.borrow().clone());
                }
                for chield in children.borrow().iter() {
                    match &**chield {
                        FsNode::File(info, _) => {
                            if by(&info.borrow()) {
                                outcome.push(info.borrow().clone());
                            }
                        }
                        FsNode::Dir(_, _, _) => {
                            self.cursor = Rc::clone(chield);
                            self.query_by_recursive(by, outcome);
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    pub fn query_by<F>(&mut self, by: F) -> Option<Vec<NodeInfo>>
    where
        F: Fn(&NodeInfo) -> bool,
    {
        let mut outcome = Vec::new();

        self.cursor = Rc::clone(&self.root);
        self.query_by_recursive(&by, &mut outcome);

        if outcome.is_empty() {
            None
        } else {
            Some(outcome)
        }
    }
}

#[derive(Debug, Clone)]
struct StateMachine<'a> {
    tree: FsTree<'a>,
    iterator: Split<'a, &'a str>,
}

impl<'a> StateMachine<'a> {
    fn new(input: &'a str) -> Self {
        let split = input.split("\n");
        let tree = FsTree::from_str("/");
        StateMachine {
            iterator: split,
            tree: tree,
        }
    }
    fn process(&mut self) -> Option<u32> {
        if let Some(text) = self.iterator.next() {
            let list = text.split_whitespace().collect::<Vec<_>>();
            return match (list[0], list[1]) {
                ("$", "cd") => Some(self.tree.change_dir(list[2])),
                ("$", "ls") => Some(1),
                ("dir", name) => Some(self.tree.add_dir(name)),
                (size, name) => Some(self.tree.add_file(name, size.parse().unwrap())),
            };
        }
        None
    }
}

#[cfg(test)]
mod unittest {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one("sample_data.txt"), Some(95437));
        assert_eq!(part_one("data.txt"), Some(1427048));
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two("sample_data.txt"), Some(24933642));
        assert_eq!(part_two("data.txt"), Some(2940614));
    }
}
