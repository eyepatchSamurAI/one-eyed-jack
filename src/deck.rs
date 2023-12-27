use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::card::{Card, Rank, Suite};

#[derive(Debug, Clone, PartialEq)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {

    /// Creates a standard 52 deck of cards with two jokers
    pub fn init_standard_with_jokers() -> Self {
        let mut cards = Deck::create_standard_deck();
        let jokers = vec![Card::new(Rank::Joker, Suite::None); 2];
        cards.extend(jokers);
        Deck { cards }
    }

    /// Creates a standard 52 deck of cards without jokers
    pub fn init_standard() -> Self {
        let cards = Deck::create_standard_deck();
        Deck { cards }
    }

    /// Creates a deck with multiple standard decks
    pub fn init_standard_multiple_decks(number_of_decks: usize) -> Self {
        let mut cards = Vec::new();
        for _ in 0..number_of_decks {
            cards.extend(Deck::create_standard_deck());
        }
        Deck { cards }
    }

    /// Draws a card from the top of the deck.
    /// If the deck is empty returns None
    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Draws an amount of cards up until the size of the deck.
    /// If you draw more than there are cards in the deck it will return early
    pub fn draw_x_cards(&mut self, amount: usize) -> Vec<Card> {
        let length = self.cards.len();
        let draw_amount = amount.min(length);

        let mut remaining_cards = self.cards.split_off(length - draw_amount);
        remaining_cards.reverse();

        remaining_cards
    }

    /// Draws from the bottom of the deck.
    /// If the deck is empty returns None
    pub fn draw_from_bottom(&mut self) -> Option<Card> {
        if self.cards.len() == 0 {
            return None;
        }
        Some(self.cards.remove(0))
    }

    /// Checks the top card of the deck
    pub fn peek(&self) -> Option<&Card> {
        self.cards.last()
    }

    /// Inserts a card at the bottom of the deck.
    pub fn insert_at_bottom(&mut self, card: Card) {
        self.cards.insert(0, card);
    }

    /// Inserts a card a certain number of places from the top of the deck.
    pub fn insert_x_from_top(&mut self, place: usize, card: Card) {
        let position = self.cards.len() - 1 - place;
        self.cards.insert(position, card);
    }

    /// Shuffles the cards in the deck.
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut thread_rng());
    }

    /// Checks if the deck is empty.
    pub fn is_deck_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Returns the number of cards in the deck.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    fn create_standard_deck() -> Vec<Card> {
        let mut cards = Vec::new();
        for suite in Suite::get_all_suites() {
            for rank in Rank::get_all_ranks_no_joker() {
                cards.push(Card::new(rank, suite));
            }
        }
        cards
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Rank;

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
