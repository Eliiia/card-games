enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

struct Card {
    suit: Suit,
    value: Value,
}

impl Card {
    fn new(suit: Suit, value: Value) -> Card {
        Card { suit, value }
    }

    fn get_suit(&self) -> &Suit {
        &self.suit
    }

    fn get_value(&self) -> &Value {
        &self.value
    }
}
