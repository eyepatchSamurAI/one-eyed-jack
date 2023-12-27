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