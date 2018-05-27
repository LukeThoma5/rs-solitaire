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

impl From<std::num::ParseIntError> for GameError {
    fn from(x: std::num::ParseIntError) -> GameError {
        GameError::Generic
    }
}

impl From<std::io::Error> for GameError {
    fn from(x: std::io::Error) -> GameError {
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

    fn reveal(&mut self) -> () {
        if self.visible.len() == 0 && self.hidden.len() > 0 {
            self.visible.push_front(self.hidden.pop_back().unwrap());
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

        move_column.reveal();

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
            let card = self.hand.last()?;

            let column = self.cols.get(column)?;

            let destination = column.visible.back();

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
            Some(card) => print!(
                "| {} {:2.0} |",
                match card.suit {
                    CardSuit::Heart => "♥",
                    CardSuit::Spade => "♠",
                    CardSuit::Diamond => "♦",
                    CardSuit::Club => "♣",
                },
                card.value
            ),
            _ => print!("| None |"),
        };
        print!("\n\n");
        self.hand
            .iter()
            .rev()
            .take(3)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .for_each(|card| print_card(Some(card)));
        print!("    ");
        print_card(self.heart.last());
        print_card(self.diamond.last());
        print_card(self.spade.last());
        print_card(self.club.last());
        print!("\n\n");
        for col in &self.cols {
            print!("|  {:2.0}  |", col.hidden.len());
        }
        print!("\n");
        for layer in 0..5 {
            for col in &self.cols {
                print_card(col.visible.iter().skip(layer).next());
            }
            print!(" \r\n")
        }
    }

    fn column_to_bucket(&mut self, column: usize) -> Result<(), GameError> {
        let column = self.cols.get_mut(column)?;
        let bucket: &mut Vec<Card>;
        {
            let move_card = column.visible.back()?;
            bucket = match move_card.suit {
                CardSuit::Heart => &mut self.heart,
                CardSuit::Club => &mut self.club,
                CardSuit::Diamond => &mut self.diamond,
                CardSuit::Spade => &mut self.spade,
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
        column.reveal();
        Ok(())
    }

    fn bucket_to_column(&mut self, column: usize, bucket_suit: CardSuit) -> Result<(), GameError> {
        let column = self.cols.get_mut(column)?;
        let bucket: &mut Vec<Card>;
        {
            bucket = match bucket_suit {
                CardSuit::Heart => &mut self.heart,
                CardSuit::Club => &mut self.club,
                CardSuit::Diamond => &mut self.diamond,
                CardSuit::Spade => &mut self.spade,
            };
            let destination_card = column.visible.back();
            let move_card = bucket.last()?;

            if !Playfield::is_allowed_move(move_card, &destination_card) {
                return Err(GameError::Generic);
            }
        }

        column.visible.push_back(bucket.pop()?);
        Ok(())
    }

    fn has_won(&self) -> bool {
        self.heart.len() == 13
            && self.diamond.len() == 13
            && self.club.len() == 13
            && self.spade.len() == 13
    }
}

fn make_move(field: &mut Playfield) -> Result<(), GameError> {
    use std::io::stdin;

    field.print();

    let mut command = String::new();
    stdin().read_line(&mut command)?;

    let tokens: Vec<_> = command.trim().split(' ').collect();

    println!("{:?}", tokens);

    match (tokens.get(0), tokens.get(1), tokens.get(2), tokens.get(3)) {
        (Some(&"mov"), Some(start_column), Some(start_index), Some(destination_column)) => {
            field.move_cards(
                start_column.parse()?,
                start_index.parse()?,
                destination_column.parse()?,
            )?;
        }
        (Some(&"draw"), ..) => field.draw_hand(),
        (Some(&"exit"), ..) => std::process::exit(0),
        (Some(&"hand"), Some(column), ..) => field.hand_to_column(column.parse()?)?,
        (Some(&"bucket"), Some(&"to"), Some(column), ..) => {
            field.column_to_bucket(column.parse()?)?
        }
        (Some(&"bucket"), Some(&"from"), Some(suit), Some(column)) => field.bucket_to_column(
            column.parse()?,
            match suit {
                &"h" => CardSuit::Heart,
                &"d" => CardSuit::Diamond,
                &"c" => CardSuit::Club,
                &"s" => CardSuit::Spade,
                _ => return Err(GameError::Generic),
            },
        )?,
        (Some(&"cheat"), Some(&"pop"), ..) => {field.backlog.push(field.hand.pop()?);},
        _ => {}
    }

    Ok(())
}

fn main() -> Result<(), GameError> {
    let mut field = Playfield::new();
    field.draw_hand();
    loop {
        println!("{:?}", field.hand);
        match make_move(&mut field) {
            Err(_) => println!("incorrect try again"),
            _ => {}
        }
        if field.has_won() {
            break;
        }
    }
    Ok(())
}
