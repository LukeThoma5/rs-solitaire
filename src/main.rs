#![feature(rustc_private)]
#![feature(proc_macro_path_invoc)]
#![feature(use_extern_macros)]
#![feature(try_trait)]
extern crate rand;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Copy)]
enum CardSuit {
    Diamond,
    Spade,
    Heart,
    Club,
}
#[derive(Debug, Clone, Copy, PartialEq)]
enum CardType {
    Red,
    Black,
}
#[derive(Debug, Clone, Copy)]
struct Card {
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

#[derive(Debug)]
struct Column {
    hidden: Vec<Card>,
    visible: Vec<Card>,
}

impl Column {
    fn new() -> Column {
        Column {
            hidden: vec![],
            visible: vec![],
        }
    }
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

        rng.shuffle(&mut cards);
        let mut iter = cards.into_iter();
        for iteration in 0..7 {
            columns[iteration].visible.push(iter.next().unwrap());
            for other in iteration + 1..7 {
                columns[other].hidden.push(iter.next().unwrap());
            }
        }

        Playfield {
            cols: columns,
            hand: vec![],
            backlog: iter.take(4).collect(),
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
        let mut move_column = self.cols.get(col_from_index)?;

        let move_card = move_column
            .visible
            .get(move_column.visible.len() - card_index)?;

        let destination_card = self.cols.get(destination)?.visible.last();

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
}

fn main() -> () {
    println!("Hello world");

    let mut field = Playfield::new();

    // println!("{:?}", field);
    println!(
        "{:?} {:?} {}",
        field.backlog,
        field.hand,
        field.backlog.len()
    );
    field.draw_hand();
    println!(
        "{:?} {:?} {}",
        field.backlog,
        field.hand,
        field.backlog.len()
    );
    field.draw_hand();
    println!(
        "{:?} {:?} {}",
        field.backlog,
        field.hand,
        field.backlog.len()
    );
    field.draw_hand();
    println!(
        "{:?} {:?} {}",
        field.backlog,
        field.hand,
        field.backlog.len()
    );
}
