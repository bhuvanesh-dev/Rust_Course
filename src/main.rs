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
}

fn main() {
    let deck = Deck::new();
    println!("Here is the deck: {:#?}", deck);
}
