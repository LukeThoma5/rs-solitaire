#![feature(rustc_private)]
#![feature(proc_macro_path_invoc)]
#![feature(use_extern_macros)]
#![feature(try_trait)]
extern crate rand;
use rand::{thread_rng, Rng};
use std::collections::LinkedList;
use std::error::Error;
use std::fmt;
#[derive(Debug, Clone, Copy, PartialEq)]
enum CardSuit {
    Diamond,
    Spade,
    Heart,
    Club,
}

#[derive(Debug, Clone, Copy)]
enum GameError {
    Generic,
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Error for GameError {}

impl From<std::option::NoneError> for GameError {
    fn from(x: std::option::NoneError) -> GameError {
        GameError::Generic
    }
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
    hidden: LinkedList<Card>,
    visible: LinkedList<Card>,
}

impl Column {
    fn new() -> Column {
        Column {
            hidden: LinkedList::new(),
            visible: LinkedList::new(),
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
            backlog: iter.collect(),
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
    ) -> Result<(), GameError> {
        let mut move_column: Option<&mut Column> = None;
        let mut destination_column: Option<&mut Column> = None;
        self.cols.iter_mut().enumerate().for_each(|(index, item)| {
            if index == col_from_index {
                move_column = Some(item)
            } else if index == destination {
                destination_column = Some(item)
            }
        });

        let (mut move_column, mut destination_column) = match (move_column, destination_column) {
            (Some(mov), Some(dest)) => (mov, dest),
            _ => return Err(GameError::Generic),
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

            let allowed = Playfield::is_allowed_move(move_card, &destination_card);
            println!("allowed {}", allowed);
            if !allowed {
                return Err(GameError::Generic);
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

    fn is_allowed_move(move_card: &Card, destination_card: &Option<&Card>) -> bool {
        match (move_card, destination_card) {
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
        }
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

    fn hand_to_column(&mut self, column: usize) -> Result<(), GameError> {
        {
            let mut card = self.hand.last()?;

            let mut column = self.cols.get(column)?;

            let mut destination = column.visible.back();

            let allowed = Playfield::is_allowed_move(card, &destination);

            if !allowed {
                return Err(GameError::Generic);
            }
        }
        self.cols
            .get_mut(column)?
            .visible
            .push_back(self.hand.pop()?);

        if self.hand.len() == 0 {
            self.draw_hand();
        }

        Ok(())
    }

    fn print(&self) {
        let print_card = |card: Option<&Card>| match card {
                    Some(card) => print!("| {} {:2.0} |", 
                    match card.suit {
                        CardSuit::Heart => "♥",
                        CardSuit::Spade => "♠",
                        CardSuit::Diamond => "♦",
                        CardSuit::Club => "♣"
                    }
                    , card.value),
                    _ => print!("| None |"),
                };
        // println!("{:?}", self.hand);
        print!("\n\n");
        self.hand.iter().take(3).for_each(|card| print_card(Some(card)));
        print!("    ");
        print_card(self.heart.last());
        print_card(self.diamond.last());
        print_card(self.spade.last());
        print_card(self.club.last());
        print!("\n\n");

        for layer in 0..5 {
            for col in &self.cols {
                print_card(col.visible.iter().skip(layer).next());
                // match col.visible.iter().skip(layer).next() {
                //     Some(card) => print!("| {:?} {:?} |", card.suit, card.value),
                //     _ => print!("| None |"),
                // }
            }
            print!(" \r\n")
        }
    }

    fn column_to_bucket(&mut self, column: usize, 
     ) -> Result<(), GameError> {
        
        let column = self.cols.get_mut(column)?;
        let bucket: &mut Vec<Card>;
        {
            let move_card = column.visible.back()?;
            bucket = match move_card.suit {
                CardSuit::Heart => &mut self.heart,
                CardSuit::Club => &mut self.club,
                CardSuit::Diamond => &mut self.diamond,
                CardSuit::Spade => &mut self.spade
            };
            
            let destination_card = bucket.last();
            if !match destination_card {
                Some(dest) => dest.value + 1 == move_card.value,
                None => move_card.value == 1,
            } {
                return Err(GameError::Generic);
            }
        }

        bucket.push(column.visible.pop_back()?);
        Ok(())
    }
}

fn main() -> Result<(), GameError> {
    println!("Hello world");

    let mut field = Playfield::new();

    // println!("{:?}", field.cols);

    field.cols[0]
        .visible
        .push_back(Card::new(9, CardSuit::Diamond));
    field.cols[1]
        .visible
        .push_back(Card::new(7, CardSuit::Heart));

    field.cols[6]
        .visible
        .push_back(Card::new(12, CardSuit::Heart));

    field.cols[2]
        .visible
        .push_back(Card::new(1, CardSuit::Heart));

    field.draw_hand();
    field.print();
    field.move_cards(1, 1, 0).unwrap();
    field.print();

    field.hand_to_column(6).unwrap();
    field.print();

    field.hand.pop()?;
    field.hand_to_column(1).unwrap();
    field.print();

    field.column_to_bucket(2).unwrap();
    field.print();
    field.column_to_bucket(2).unwrap();
    field.print();
    Ok(())
}
