use std::cell::Cell;

// TODO: Make this into an Iter
pub struct Players {
    items: Vec<Player>,
    current: Cell<usize>,
}

impl Players {
    pub fn new(number_of_players: u32) -> Players {
        Players {
            items: (0..number_of_players).map(Player::new).collect(),
            current: Cell::new(0),
        }
    }

    pub fn get_next(&self) -> &Player {
        let player = &self.items.get(self.current.get()).unwrap_or_else(|| {
            self.current.set(0);
            &self.items[self.current.get()]
        });
        self.current.update(|v| v + 1);
        player
    }
}

#[derive(Debug)]
pub struct Player {
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

    pub fn add_score(&self, points: u32) {
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
        let next_players: Vec<&Player> = (0..5).map(|_| players.get_next()).collect();

        // Then

        // verify that it rotates correctly
        assert_eq!(next_players[3].id, next_players[0].id);
    }

    #[test]
    fn players_have_different_ids() {
        // Given
        let players = Players::new(3);

        // When
        let next_players: Vec<&Player> = (0..3).map(|_| players.get_next()).collect();

        // Then
        assert_eq!(0, next_players[0].id);
        assert_eq!(1, next_players[1].id);
        assert_eq!(2, next_players[2].id);
    }
}
