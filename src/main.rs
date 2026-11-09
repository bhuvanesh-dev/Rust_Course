#[derive(Debug)]
struct Deck{
    cards: Vec<String>
}

fn main() {
    let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
    let ranks = ["Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King"];
    let mut cards = Vec::new();
    for suit in suits{
        for rank in ranks{
            let card = format!("{} of {}", rank, suit);
            cards.push(card);
        }
    }
    let deck = Deck{ cards: cards};
    println!("Here is the deck: {:#?}", deck);
}
