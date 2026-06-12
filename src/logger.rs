use std::cell::RefCell;
use std::rc::Rc;

pub fn run() {
    let logs: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![]));

    let module_a = Rc::clone(&logs);

    let module_b = Rc::clone(&logs);

    module_a
        .borrow_mut()
        .push("Module A: 起動しました".to_string());

    module_b
        .borrow_mut()
        .push("Module B: 起動しました".to_string());

    logs.borrow().iter().for_each(|log| println!("{}", log));
}
