use std::{collections::HashMap, str::FromStr};

fn rockPaperScissors() {
    // set up the base score for each play
    let mut base_score = HashMap::new();
    base_score.insert('X', 1); // rock
    base_score.insert('Y', 2); // paper
    base_score.insert('Z', 3); // scissors

    // set up who beats who
    let mut who_beats = HashMap::new();
    who_beats.insert('A', 'Y'); // paper beats rock
    who_beats.insert('B', 'Z'); // scissors beats paper
    who_beats.insert('C', 'X'); // rock beats scissors

    // set up who ties with who
    let mut who_ties_with = HashMap::new();
    who_ties_with.insert('A', 'X'); 
    who_ties_with.insert('B', 'Y'); 
    who_ties_with.insert('C', 'Z'); 
    
    // the following line is good for rust, because
    // it brings the input into the binary when the program compiles
    let input = include_str!("input_day2.txt");

    let mut score = 0;
    for round in input.lines() {
        let plays = Play::from_str(round).unwrap();

        score += base_score.get(&plays.me).unwrap();

        if plays.me == *who_beats.get(&plays.other).unwrap() {
            score += 6; 
        } else if plays.me == *who_ties_with.get(&plays.other).unwrap(){
            score += 3
        }
    }

    println!("{score}");
    
}

pub struct Play {
    me: char,
    other: char,
}

impl std::str::FromStr for Play {
    type Err = char;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.chars();
        let other = it.next().unwrap();
        it.next();
        let me = it.next().unwrap();
        Ok(Play {
            me,
            other
        })
    }
}