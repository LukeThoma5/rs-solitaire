// mod super::MCard;
use super::MCard::Card;
// mod MCardSuit;
// mod MCardType;
use super::MCardSuit::CardSuit;
use super::MCardType::MCardType;

pub mod MPlayfield {
    #[derive(Debug)]
    struct Playfield {
        cols: Vec<Column>,
        backlog: Vec<Card>,
        hand: Vec<Card>,
        heart: Vec<Card>,
        spade: Vec<Card>,
        club: Vec<Card>,
        diamond: Vec<Card>,
        hand_index: usize,
    }

    impl Playfield {
        fn new() -> Playfield {
            let mut rng = thread_rng();
            let mut columns = vec![];
            for _ in 0..7 {
                columns.push(Column::new());
            }

            let mut cards = Vec::with_capacity(52);
            for suit in vec![
                CardSuit::Club,
                CardSuit::Diamond,
                CardSuit::Heart,
                CardSuit::Spade,
            ] {
                for value in 1..=13 {
                    cards.push(Card::new(value, suit));
                }
            }

            // rng.shuffle(&mut cards);
            let mut iter = cards.into_iter();
            for iteration in 0..7 {
                columns[iteration].visible.push_back(iter.next().unwrap());
                for other in iteration + 1..7 {
                    columns[other].hidden.push_back(iter.next().unwrap());
                }
            }

            Playfield {
                cols: columns,
                hand: vec![],
                backlog: iter.take(0).collect(),
                heart: Vec::with_capacity(13),
                spade: Vec::with_capacity(13),
                club: Vec::with_capacity(13),
                diamond: Vec::with_capacity(13),
                hand_index: 0,
            }
        }

        fn move_cards(
            &mut self,
            col_from_index: usize,
            card_index: usize,
            destination: usize,
        ) -> Result<(), Box<std::option::NoneError>> {
            let mut move_column: Option<&mut Column> = None;
            let mut destination_column: Option<&mut Column> = None;
            self.cols.iter_mut().enumerate().for_each(|(index, item)| {
                if index == col_from_index {
                    move_column = Some(item)
                } else if index == destination {
                    destination_column = Some(item)
                }
            });

            let (mut move_column, mut destination_column) = match (move_column, destination_column)
            {
                (Some(mov), Some(dest)) => (mov, dest),
                _ => return Err(Box::new(std::option::NoneError)),
            };

            println!("Found columns");

            let move_column_len = move_column.visible.len();

            {
                let move_card = move_column
                    .visible
                    .iter()
                    .skip(move_column_len - card_index - 1)
                    .next()?;

                println!("Found move card {:?}", move_card);

                let destination_card = destination_column.visible.back();

                println!("Found destination {:?}", destination_card);

                let allowed = match (move_card, destination_card) {
                    (Card { value: 13, .. }, None) => true,
                    (
                        Card {
                            card_type: start_type,
                            value,
                            ..
                        },
                        Some(Card {
                            card_type: destination_type,
                            value: destination_value,
                            ..
                        }),
                    ) if *destination_value == value + 1 =>
                    {
                        *start_type != *destination_type
                    }

                    _ => false,
                };
                println!("allowed {}", allowed);
                if !allowed {
                    return Err(Box::new(std::option::NoneError));
                }
            }
            println!("Adding items");
            let mut to_add = move_column
                .visible
                .split_off(move_column_len - card_index - 1);

            println!("items to add {:?}", to_add);

            destination_column.visible.append(&mut to_add);

            Ok(())
        }

        fn draw_hand(&mut self) {
            match self.backlog.pop() {
                Some(card) => {
                    self.hand.push(card);
                    for _ in 0..2 {
                        match self.backlog.pop() {
                            Some(card) => self.hand.push(card),
                            None => break,
                        }
                    }
                }
                None => {
                    self.backlog = Vec::clone(&self.hand).into_iter().rev().collect();
                    self.hand.clear();
                }
            }
        }

        fn print(&self) {
            for layer in 0..5 {
                for col in &self.cols {
                    match col.visible.iter().skip(layer).next() {
                        Some(card) => print!("| {:?} {:?} |", card.suit, card.value),
                        _ => print!("| None |"),
                    }
                }
                print!(" \r\n")
            }
        }
    }
}
