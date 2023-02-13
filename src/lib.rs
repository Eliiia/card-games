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

    fn get_value_as_int(&self) -> u8 {
        match self.value {
            Value::Ace => 1,
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 10,
            Value::Queen => 10,
            Value::King => 10,
        }
    }
}