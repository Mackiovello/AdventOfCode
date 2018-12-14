use super::circle::Circle;
use super::marble::{Marble, Marbles};
use super::players::Players;

struct MarbleGame {
    players: Players,
    marbles: Marbles,
    circle: Circle,
    current_marble: u32,
}

impl MarbleGame {
    fn new(number_of_players: u32, marbles: u32) -> MarbleGame {
        MarbleGame {
            players: Players::new(number_of_players),
            marbles: Marbles::new(marbles),
            circle: Circle::new(),
            current_marble: 0,
        }
    }

    fn take_turn(&self) {
        let player = self.players.get_next();
        let marble = self.marbles.get_next();

        if marble.get_value() % 23 == 0 {
            player.add_score(marble.get_value())

            // remove marble that's seven
            // steps counter-clockwise

            // add points of removed marble
            // to player

            // set marble clockwise to the
            // removed marble as the
            // current marble
        }
    }

    fn place_marble(&self, marble: Marble) {
        // place the marble between the marble that
        // are between 1 and 2 steps clockwise

        // set the marble as the current marble
    }
}
