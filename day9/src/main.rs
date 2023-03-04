// use regex::Regex;
use itertools::Itertools;
use std::fs;

fn main() {
    println!("Part_one: {}", part_one("sample_data.txt").unwrap());
    // println!("Part_one: {}", part_one("data.txt").unwrap());
    // println!("Part_two: {}", part_two("sample_data_larger.txt").unwrap());
    // println!("Part_two: {}", part_two("data.txt").unwrap());
}

fn part_one(file: &str) -> Option<u32> {
    let data = fs::read_to_string(file).unwrap();
    let mut rope = Rope::new((0, 0), 2);
    for line in data.split("\n") {
        rope.motion(line);
    }
    rope.render();
    Some(rope.knots.last().unwrap().history.iter().unique().count() as u32)
}

fn part_two(file: &str) -> Option<u32> {
    let data = fs::read_to_string(file).unwrap();
    let mut rope = Rope::new((0, 0), 10);
    for line in data.split("\n") {
        rope.motion(line);
    }
    rope.render();
    Some(rope.knots.last().unwrap().history.iter().unique().count() as u32)
}

#[derive(Debug, Clone, PartialEq)]
struct Knot {
    name: char,
    pos: (i32, i32),
    history: Vec<(i32, i32, char)>,
}
#[derive(Debug, Clone)]
enum Movement {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
    Diagonal(i32, i32),
}

impl Knot {
    fn new(pos: (i32, i32), name: char) -> Self {
        Knot {
            name,
            pos,
            history: vec![(pos.0, pos.1, name)],
        }
    }
    fn motion(&mut self, direction: Movement) {
        self.pos = match direction {
            Movement::Diagonal(x, y) => (self.pos.0 + x, self.pos.1 + y),
            Movement::Up(num) => (self.pos.0, self.pos.1 + num),
            Movement::Down(num) => (self.pos.0, self.pos.1 - num),
            Movement::Left(num) => (self.pos.0 - num, self.pos.1),
            Movement::Right(num) => (self.pos.0 + num, self.pos.1),
        };
        self.history.push((self.pos.0, self.pos.1, self.name));
    }
}
struct Rope {
    knots: Vec<Knot>,
}

impl Rope {
    fn new(start: (i32, i32), knots_num: u32) -> Self {
        let mut knots = Vec::new();
        knots.push(Knot::new(start, 'H'));
        for i in 1..knots_num - 1 {
            knots.push(Knot::new(start, std::char::from_digit(i, 10).unwrap()));
        }
        knots.push(Knot::new(start, 'T'));
        Rope { knots }
    }

    fn get_tail(&self) -> &Knot {
        &self.knots.last().unwrap()
    }

    fn compute_motion(x: i32, y: i32) -> (i32, i32) {
        let dir_x = x.signum();
        let x = x.abs();
        let dir_y = y.signum();
        let y = y.abs();
        match (x+y, x, y) {
            (3, _, _) => (1 * dir_x, 1 * dir_y),
            (_, 0, z) =>  (0, (z - 1) * dir_y ),
            (_, z, 0) =>  ((z - 1) * dir_x, 0 ),
            _ =>  ((x - 1) * dir_x, (y - 1) * dir_y ),
        }
    }

    fn motion(&mut self, command: &str) {
        let (dir, steps) = command.split_once(' ').unwrap();
        let steps = steps.parse().unwrap();
        let dir = match dir {
            "R" => Movement::Right(1),
            "L" => Movement::Left(1),
            "U" => Movement::Up(1),
            "D" => Movement::Down(1),
            cmd => panic!("Unknown cmd {cmd}"),
        };

        for _ in 0..steps {
            let mut iter = self.knots.iter_mut();

            let mut previous = iter.next().unwrap();
            previous.motion(dir.clone());

            while let Some(current) = iter.next() {
                let deviation = previous.sub(&current);
                let (x, y) = Rope::compute_motion(deviation.0, deviation.1);
                current.motion(Movement::Diagonal(x, y));

                previous = current;
            }
        }
    }
}

impl Knot {
    fn sub(&self, other: &Self) -> (i32, i32) {
        (self.pos.0 - other.pos.0, self.pos.1 - other.pos.1)
    }
}

impl Rope {
    fn render(&self) {
        let head = &self.knots[0];
        let max_x = head.history.iter().map(|x| x.0).max().unwrap();
        let min_x = head.history.iter().map(|x| x.0).min().unwrap();
        let max_y = head.history.iter().map(|x| x.1).max().unwrap();
        let min_y = head.history.iter().map(|x| x.1).min().unwrap();

        println!(
            "x max:{}, x min:{} y max:{}, y min:{} ",
            max_x, min_x, max_y, min_y
        );

        let mut pos_iter = self
            .knots
            .iter()
            .map(|x| x.history.iter())
            .collect::<Vec<_>>();

        for _ in 0..head.history.len() {
<<<<<<< Updated upstream
            for y in (min_y..=max_y).rev() {
                for x in min_x..=max_x {
                    let mut pixel = '.';
                    for iter in pos_iter.iter_mut().rev() {
                        let knot = iter.peek().unwrap();
                        if (x, y) == (knot.0, knot.1) {
                            pixel = knot.2;
                        }
                    }
                    // print!("{pixel} ");
                }
                // print!("\n");
            }
            // print!("\x1b[{}A", max_y - min_y + 1);
            // std::thread::sleep(std::time::Duration::from_millis(1));
            // print!("\n\n");

            for iter in pos_iter.iter_mut() {
                let _ = iter.next();
            }
=======
            let time_slice = pos_iter
                .iter_mut()
                .map(|x| x.next().unwrap())
                .collect::<Vec<_>>();
            Rope::render_frame(min_y, max_y, min_x, max_x, time_slice);
            print!("\x1b[{}A", max_y - min_y + 1);
            std::thread::sleep(std::time::Duration::from_millis(500));
            // print!("\n\n");
>>>>>>> Stashed changes
        }
        print!("\n\n");
    }

    fn render_frame(min_y: i32, max_y: i32, min_x: i32, max_x: i32, knots: Vec<&(i32, i32, char)>) {
        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                let mut pixel = '.';
                for knot in knots.iter().rev() {
                    if (x, y) == (knot.0, knot.1) {
                        pixel = knot.2;
                    }
                }
                print!("{pixel} ");
            }
            print!("\n");
        }
    }
}

#[cfg(test)]
#[macro_use]

#[macro_use]
extern crate time_test;
#[cfg(test)]
mod unittest {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("sample_data.txt"), Some(13));
        assert_eq!(part_one("data.txt"), Some(5619));
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two("sample_data.txt"), Some(1));
        assert_eq!(part_two("data.txt"), Some(2376));
    }

    #[test]
    fn test_banchmark() {
        time_test!("Part one");
<<<<<<< Updated upstream
        for _ in 0..1 {
=======
        for _ in 0..1000 {
>>>>>>> Stashed changes
            assert_eq!(part_one("data.txt"), Some(5619));
        }
        {
            time_test!("Part two");
<<<<<<< Updated upstream
            for _ in 0..1 {
=======
            for _ in 0..1000 {
>>>>>>> Stashed changes
                assert_eq!(part_two("data.txt"), Some(2376));
            }
        }
    }

    #[test]
    fn test_next_mov() {
        assert_eq!(Rope::compute_motion(0,0), (0,0));
        assert_eq!(Rope::compute_motion(1,0), (0,0));
        assert_eq!(Rope::compute_motion(0,1), (0,0));
        assert_eq!(Rope::compute_motion(1,1), (0,0));
        assert_eq!(Rope::compute_motion(2,0), (1,0));
        assert_eq!(Rope::compute_motion(0,2), (0,1));
        assert_eq!(Rope::compute_motion(2,1), (1,1));
        assert_eq!(Rope::compute_motion(1,2), (1,1));

        assert_eq!(Rope::compute_motion(0,0), (0,0));
        assert_eq!(Rope::compute_motion(-1,0), (0,0));
        assert_eq!(Rope::compute_motion(0,-1), (0,0));
        assert_eq!(Rope::compute_motion(-1,-1), (0,0));
        assert_eq!(Rope::compute_motion(-1,1), (0,0));
        assert_eq!(Rope::compute_motion(1,-1), (0,0));
        assert_eq!(Rope::compute_motion(-2,0), (-1,0));
        assert_eq!(Rope::compute_motion(0,-2), (0,-1));
        assert_eq!(Rope::compute_motion(-2,-1), (-1,-1));
        assert_eq!(Rope::compute_motion(-2,1), (-1,1));
        assert_eq!(Rope::compute_motion(2,-1), (1,-1));
        assert_eq!(Rope::compute_motion(-1,-2), (-1,-1));
        assert_eq!(Rope::compute_motion(-1,2), (-1,1));
        assert_eq!(Rope::compute_motion(1,-2), (1,-1));
    }
}
