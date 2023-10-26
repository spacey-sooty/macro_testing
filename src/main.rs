#[macro_export]
macro_rules! map {
    ( $( $x:expr, $y:expr ),* ) => {
        {
            let mut temp_map = std::collections::HashMap::new();
            $(
                let mut count = 0;
                for i in $x {
                    temp_map.insert(i, $y[count]);
                    count += 1;
                }
            )*
            temp_map
        }
    };
}

fn main() {
    let map = map!([1, 2, 3],["hello", "joe", "mum"]);
    for (key, value) in map {
        println!("\n{:#?}: {:#?}", key, value);
    }
}

