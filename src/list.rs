pub enum List {
    Cons(i32, Box<List>),
    Nil,
}
impl List {
    fn sum(&self) -> i32 {
        match self {
            List::Cons(num, list) => *num + list.sum(),
            List::Nil => 0,
        }
    }
}
