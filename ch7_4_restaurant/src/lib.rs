#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;

// fn function1() -> Result {

// }

// fn function2() -> IoResult<()> {
//     IoResult<()>
// }

fn use_hashmap_function() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
