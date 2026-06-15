#[macro_export]
macro_rules! hashmap {
    ($($k:expr => $v:expr),*) => {
        {
            use std::collections::HashMap;
            let mut m = HashMap::new();
            $(m.insert($k,$v);)*
            m
        }
    };
}
