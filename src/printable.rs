pub trait Printable {
    fn category(&self) -> String;
    fn display(&self) -> String;

    fn full_info(&self) -> String {
        format!("{} {}", self.category(), self.display())
    }
}

pub fn print_item<T: Printable>(item: &T) {
    println!("{}", item.full_info());
}

pub fn print_item_other(item: &impl Printable) {
    println!("{}", item.full_info());
}
