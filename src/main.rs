use rand::{ rng, seq::SliceRandom };

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    // This is a constructor function for the Deck struct. It creates a new deck of cards.
    // The function is defined as a static method (associated function) of the Deck struct, which means it can be called without an instance of Deck.
    fn new() -> Deck {
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
        let ranks = ["Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King"];
        let mut cards = Vec::new();
        for suit in suits {
            for rank in ranks {
                let card = format!("{} of {}", rank, suit);
                cards.push(card);
            }
        }
        // let deck = Deck { cards: cards };
        // return deck;

        // Alternatively, we can return the Deck directly without creating a separate variable,
        //  this is called implicit return in Rust and the statement don't end with a semicolon.
        Deck { cards: cards }
    }

    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, numOfCards: usize) -> Vec<String> {
        // This method takes a mutable reference to self and a number of cards to deal.
        // It uses the split_off method to remove the specified number of cards from the end of the deck and return them as a new vector.
        // The split_off method takes an index as an argument and splits the vector into two at that index.
        // The original vector retains the elements before the index, and the new vector contains the elements from the index to the end.
        // In this case, we want to split the vector at the index that is numOfCards from the end of the vector, which is calculated as self.cards.len() - numOfCards.
        self.cards.split_off(self.cards.len() - numOfCards)
    }
}

fn main() {
    let mut deck = Deck::new();
    //deck.shuffle();
    let dealt_cards = deck.deal(5);
    println!("Here are the dealt cards: {:#?}", dealt_cards);
    println!("Here is the deck: {:#?}", deck);
}
