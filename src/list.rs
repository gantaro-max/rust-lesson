pub enum List {
    Cons(i32, Box<List>),
    Nil,
}
impl List {
    pub fn sum(&self) -> i32 {
        match self {
            List::Cons(num, list) => *num + list.sum(),
            List::Nil => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum() {
        let test_list = List::Cons(
            1,
            Box::new(List::Cons(
                2,
                Box::new(List::Cons(
                    3,
                    Box::new(List::Cons(4, Box::new(List::Cons(5, Box::new(List::Nil))))),
                )),
            )),
        );

        assert_eq!(test_list.sum(), 15);
    }
}
