use std::{collections::HashMap, str::FromStr};

fn rockPaperScissors(){
    // X: loose, Y: draw, Z: win 
    // A: rock, B: paper, C: scissors
    // set up the base score for each play
    let mut base_score = HashMap::new();
    base_score.insert('A', 1); // rock
    base_score.insert('B', 2); // paper
    base_score.insert('C', 3); // scissors

    // set up who beats who
    let mut who_beats = HashMap::new();
    who_beats.insert('A', 'B'); // paper beats rock
    who_beats.insert('B', 'C'); // scissors beats paper
    who_beats.insert('C', 'A'); // rock beats scissors

    // set up who loses to who
    let mut who_loses_to = HashMap::new();
    who_loses_to.insert('A', 'C'); 
    who_loses_to.insert('B', 'A'); 
    who_loses_to.insert('C', 'B'); 
    
    // the following line is good for rust, because
    // it brings the input into the binary when the program compiles
    let input = include_str!("input_day2.txt");

    let mut score = 0;
    for round in input.lines() {
        let plays = Play::from_str(round).unwrap();

        match plays.me {
            // Loose
            'X' => {
                let loosing_play = *who_loses_to.get(&plays.other).unwrap();
                score += base_score.get(&loosing_play).unwrap();

            },

            // Draw
            'Y' => {
                score += base_score.get(&plays.other).unwrap() + 3;
            },

            // Win
            'Z' => {
                let winning_play = *who_beats.get(&plays.other).unwrap();
                score += base_score.get(&winning_play).unwrap() + 6;
            },

            _ => continue
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