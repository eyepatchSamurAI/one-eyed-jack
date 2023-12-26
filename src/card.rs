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
}

impl Rank {
    pub fn get_ranks() -> [Rank; 13] {
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
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Suite {
    Heart,
    Diamond,
    Club,
    Spade,
}

impl Suite {
    pub fn get_suites() -> [Suite; 4] {
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