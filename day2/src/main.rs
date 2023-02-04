use std::fs;
use std::ops::Add;

fn main() {
    println!("Hello, world!");
    part_one();
    part_two();
}

#[derive(Debug, Copy, Clone)]
enum Tools{
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, Copy, Clone)]
enum Outcome{
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl Add<Tools> for u32 {
    type Output = Self;

    fn add(self, other: Tools) -> Self {
        self + other as u32
    }
}
impl Add<Outcome> for u32 {
    type Output = Self;

    fn add(self, other: Outcome) -> Self {
        self + other as u32
    }
}
impl Add<Outcome> for Tools {
    type Output = u32;

    fn add(self, other: Outcome) -> u32 {
        self as u32 + other as u32
    }
}
impl Add<Tools> for Outcome {
    type Output = u32;

    fn add(self, other: Tools) -> u32 {
        self as u32 + other as u32
    }
}

fn part_one(){
    let data = fs::read_to_string("data.txt").unwrap();
    let mut player1_total_score = 0u32;
    let mut player2_total_score = 0u32;

    for line in data.split("\n"){
        let play:Vec<_> = line.split(" ").collect::<Vec<_>>();
        let first_player_tool = match play[0] {
            "A" => Tools::Rock,
            "B" => Tools::Paper,
            "C" => Tools::Scissors,
              _ => panic!("Undefined")
        };
        let second_player_tool = match play[1] {
            "X" => Tools::Rock,
            "Y" => Tools::Paper,
            "Z" => Tools::Scissors,
              _ => panic!("Undefined")
        };

        player1_total_score = player1_total_score + first_player_tool;
        player2_total_score = player1_total_score + second_player_tool;

        let (player1_score, player2_score) = match (first_player_tool, second_player_tool){
            (Tools::Rock,     Tools::Paper) => (Outcome::Lose, Outcome::Win),
            (Tools::Rock,     Tools::Scissors) => (Outcome::Win, Outcome::Lose),
            (Tools::Rock,     Tools::Rock) => (Outcome::Draw,  Outcome::Draw),
   
            (Tools::Paper,    Tools::Rock) => ( Outcome::Win, Outcome::Lose),
            (Tools::Paper,    Tools::Scissors) => (Outcome::Lose, Outcome::Win),
            (Tools::Paper,    Tools::Paper) => (Outcome::Draw, Outcome::Draw),
            
            (Tools::Scissors, Tools::Rock) => (Outcome::Lose, Outcome::Win),
            (Tools::Scissors, Tools::Paper) => (Outcome::Win, Outcome::Lose),
            (Tools::Scissors, Tools::Scissors) => (Outcome::Draw, Outcome::Draw),
        };
        player1_total_score = player1_total_score + player1_score;
        player2_total_score = player2_total_score + player2_score;
    }
    println!("total p1:{:#?} vs total p2:{:#?}", player1_total_score, player2_total_score);
}

fn part_two(){
    let data = fs::read_to_string("data.txt").unwrap();
    let mut player1_total_score = 0u32;
    let mut player2_total_score = 0u32;

    for line in data.split("\n"){
        let play:Vec<_> = line.split(" ").collect::<Vec<_>>();
        let first_player_tool = match play[0] {
            "A" => Tools::Rock,
            "B" => Tools::Paper,
            "C" => Tools::Scissors,
            &_ => panic!("Undefined")
        };
        let second_player_tool = match play[1] {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            &_ => panic!("Undefined")
        };

        let (player1_score, player2_score) = match (first_player_tool, second_player_tool){
            (Tools::Rock,     Outcome::Lose) => (Outcome::Win  + Tools::Rock , Outcome::Lose + Tools::Scissors ),
            (Tools::Rock,     Outcome::Draw) => (Outcome::Draw + Tools::Rock , Outcome::Draw + Tools::Rock),
            (Tools::Rock,     Outcome::Win) =>  (Outcome::Lose + Tools::Rock , Outcome::Win + Tools::Paper),
   
            (Tools::Paper,    Outcome::Lose) => (Outcome::Win  + Tools::Paper, Outcome::Lose + Tools::Rock),
            (Tools::Paper,    Outcome::Draw) => (Outcome::Draw + Tools::Paper, Outcome::Draw + Tools::Paper),
            (Tools::Paper,    Outcome::Win)  => (Outcome::Lose + Tools::Paper, Outcome::Win  + Tools::Scissors),
                        
            (Tools::Scissors, Outcome::Lose) => (Outcome::Win  + Tools::Scissors, Outcome::Lose + Tools::Paper),
            (Tools::Scissors, Outcome::Draw) => (Outcome::Draw + Tools::Scissors, Outcome::Draw + Tools::Scissors),
            (Tools::Scissors, Outcome::Win)  => (Outcome::Lose + Tools::Scissors, Outcome::Win  + Tools::Rock),
        };
        player1_total_score += player1_score;
        player2_total_score += player2_score;
    }
    println!("total p1:{:#?} vs total p2:{:#?}", player1_total_score, player2_total_score);
}