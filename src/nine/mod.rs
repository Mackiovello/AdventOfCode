#![allow(dead_code)]

use std::cell::Cell;
use std::collections::LinkedList;

pub fn problem_nine_part_one() -> u32 {
    // No input file, these are the numbers
    // we get instead
    let players = 404;

    // marbles are zero indexed, so
    // the number of marbles is one
    // more than this
    let last_marble_points = 71852;
    0
}

struct MarbleGame {
    players: Players,
    marbles: Vec<Marble>,
    circle: Vec<Marble>,
    current_marble: u32,
}

impl MarbleGame {
    fn new(number_of_players: u32, marbles: u32) -> MarbleGame {
        MarbleGame {
            players: Players::new(number_of_players),
            marbles: (0..marbles).map(Marble::new).collect(),
            circle: Vec::new(),
            current_marble: 0,
        }
    }

    fn take_turn(&self) {
        let player = self.players.get_next_player();
        let marble = self.get_next_marble();

        if marble.value % 23 == 0 {
            player.add_score(marble.value)

            // remove marble that's seven
            // steps counter-clockwise

            // add points of removed marble
            // to player

            // set marble clockwise to the
            // removed marble as the
            // current marble
        }
    }

    fn get_next_marble(&self) -> &Marble {
        // TODO: pick the next marble properly
        &self.marbles[0]
    }

    fn place_marble(&self, marble: Marble) {
        // place the marble between the marble that
        // are between 1 and 2 steps clockwise

        // set the marble as the current marble
    }
}

struct Marble {
    value: u32,
}

impl Marble {
    fn new(value: u32) -> Marble {
        Marble { value }
    }
}

// TODO: Make this into an Iter
struct Players {
    individuals: Vec<Player>,
    current_player: Cell<usize>,
}

impl Players {
    fn new(number_of_players: u32) -> Players {
        Players {
            individuals: (0..number_of_players).map(Player::new).collect(),
            current_player: Cell::new(0),
        }
    }

    fn get_next_player(&self) -> &Player {
        let player = &self
            .individuals
            .get(self.current_player.get())
            .unwrap_or_else(|| {
                self.current_player.set(0);
                &self.individuals[self.current_player.get()]
            });
        self.current_player.update(|v| v + 1);
        player
    }
}

#[derive(Debug)]
struct Player {
    id: u32,
    score: Cell<u32>,
}

impl Player {
    fn new(id: u32) -> Player {
        Player {
            id,
            score: Cell::new(0),
        }
    }

    fn add_score(&self, points: u32) {
        self.score.update(|s| s + points);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_next_player_returns_after_rotating_through_all_players() {
        // Given
        let players = Players::new(3);

        // When
        let next_players: Vec<&Player> = (0..5).map(|_| players.get_next_player()).collect();

        // Then

        // verify that it rotates correctly
        assert_eq!(next_players[3].id, next_players[0].id);
    }

    #[test]
    fn players_have_different_ids() {
        // Given
        let players = Players::new(3);

        // When
        let next_players: Vec<&Player> = (0..3).map(|_| players.get_next_player()).collect();

        // Then
        assert_eq!(0, next_players[0].id);
        assert_eq!(1, next_players[1].id);
        assert_eq!(2, next_players[2].id);
    }
}
