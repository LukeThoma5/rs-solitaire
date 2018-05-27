// mod MCardSuit;
// mod MCardType;
use super::MCardSuit::CardSuit;
use super::MCardType::CardType;

pub mod MCard {
    #[derive(Debug, Clone, Copy)]
    pub struct Card {
        value: u8,
        suit: CardSuit,
        card_type: CardType,
    }

    impl Card {
        fn new(value: u8, suit: CardSuit) -> Card {
            Card {
                value,
                suit,
                card_type: match suit {
                    CardSuit::Club | CardSuit::Spade => CardType::Black,
                    _ => CardType::Red,
                },
            }
        }
    }
}
