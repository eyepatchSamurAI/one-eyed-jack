use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

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
    rank: Rank,
    suite: Suite,
}

impl Card {
    pub fn new(rank: Rank, suite: Suite) -> Self {
        Card { rank, suite }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn init_standard() -> Self {
        let mut cards = Vec::new();
        for suite in Suite::get_suites() {
            for rank in Rank::get_ranks() {
                cards.push(Card::new(rank, suite));
            }
        }
        Deck { cards }
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn draw_x_cards(&mut self, amount: usize) -> Vec<Card> {
        let length = self.cards.len();
        let draw_amount = amount.min(length);

        let mut remaining_cards = self.cards.split_off(length - draw_amount);
        remaining_cards.reverse();

        remaining_cards
    }

    pub fn draw_from_bottom(&mut self) -> Option<Card> {
        if self.cards.len() == 0 {
            return None;
        }
        Some(self.cards.remove(0))
    }

    pub fn peek(&self) -> Option<&Card> {
        self.cards.last()
    }

    pub fn insert_at_bottom(&mut self, card: Card) {
        self.cards.insert(0, card);
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut thread_rng());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw() {
        let mut deck = Deck::init_standard();
        let card = deck.draw();
        assert_eq!(deck.cards.len(), 51);
        assert_eq!(
            card.unwrap(),
            Card {
                rank: Rank::Ace,
                suite: Suite::Spade
            }
        )
    }

    #[test]
    fn test_draw_x_cards() {
        let mut deck = Deck::init_standard();
        let mut cards = deck.draw_x_cards(2);
        assert_eq!(
            cards,
            [
                Card {
                    rank: Rank::Ace,
                    suite: Suite::Spade
                },
                Card {
                    rank: Rank::Two,
                    suite: Suite::Spade
                }
            ]
        );
        assert_eq!(deck.cards.len(), 50);
        cards = deck.draw_x_cards(49);
        assert_eq!(deck.cards.len(), 1);
        cards = deck.draw_x_cards(10);
        assert_eq!(deck.cards.len(), 0);
        assert_eq!(
            cards,
            [Card {
                rank: Rank::King,
                suite: Suite::Heart
            }]
        );
    }

    #[test]
    fn test_insert_at_bottom() {
        let mut deck = Deck::init_standard();
        let card = deck.draw().unwrap();
        deck.insert_at_bottom(card);
        assert_eq!(deck.cards.first(), Some(&card.clone()));
    }
}
