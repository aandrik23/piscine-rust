use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let value = rand::thread_rng().gen_range(1..=4);

        Suit::translate(value)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            _ => Suit::Club,
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let value = rand::thread_rng().gen_range(1..=13);

        Rank::translate(value)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            n => Rank::Number(n),
        }
    }
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    matches!(card.suit, Suit::Spade)
        && matches!(card.rank, Rank::Ace)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
    let your_card = Card {
        rank: Rank::random(),
        suit: Suit::random(),
    };

    println!("Your card is {:?}", &your_card);

    if card_deck::winner_card(&your_card) {
        println!("You are the winner!");
    }
}
}