use std::collections::HashMap;
//When using a struct we use it with the full path just as a convention
fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
	    println!("Added to waitlist");	
	}
    }
}

// This approach is good but using this in eat_at_restaurant makes it look like its a local function of eat_at_restaurant. Hence, for functions we only bring the paths in scope by using the parent module. (Just a convention)
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}

fn main() {
    eat_at_restaurant(); 
}
