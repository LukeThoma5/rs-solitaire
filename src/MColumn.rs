pub mod MColumn {

    #[derive(Debug)]
pub struct Column {
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
}