use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Rank {
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Ace,
    Joker
}

impl Rank {
    pub fn get_all_ranks_no_joker() -> [Rank; 13] {
        [
            Rank::King,
            Rank::Queen,
            Rank::Jack,
            Rank::Ten,
            Rank::Nine,
            Rank::Eight,
            Rank::Seven,
            Rank::Six,
            Rank::Five,
            Rank::Four,
            Rank::Three,
            Rank::Two,
            Rank::Ace,
        ]
    }

    pub fn get_all_ranks() -> [Rank; 14] {
        [
            Rank::King,
            Rank::Queen,
            Rank::Jack,
            Rank::Ten,
            Rank::Nine,
            Rank::Eight,
            Rank::Seven,
            Rank::Six,
            Rank::Five,
            Rank::Four,
            Rank::Three,
            Rank::Two,
            Rank::Ace,
            Rank::Joker
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Suite {
    Heart,
    Diamond,
    Club,
    Spade,
    None,
}

impl Suite {
    pub fn get_all_suites() -> [Suite; 4] {
        [Suite::Heart, Suite::Diamond, Suite::Club, Suite::Spade]
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Card {
    pub rank: Rank,
    pub suite: Suite,
}

impl Card {
    pub fn new(rank: Rank, suite: Suite) -> Self {
        Card { rank, suite }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank = match self.rank {
            Rank::King => "K",
            Rank::Queen => "Q",
            Rank::Jack => "J",
            Rank::Ten => "10",
            Rank::Nine => "9",
            Rank::Eight => "8",
            Rank::Seven => "7",
            Rank::Six => "6",
            Rank::Five => "5",
            Rank::Four => "4",
            Rank::Three => "3",
            Rank::Two => "2",
            Rank::Ace => "A",
            Rank::Joker => "Joker",
        };

        let suite = match self.suite {
            Suite::Heart => "♥️",
            Suite::Diamond => "♦️",
            Suite::Club => "♣️",
            Suite::Spade => "♠️",
            Suite::None => "",
        };

        write!(f, "{}{}", rank, suite)
    }
}