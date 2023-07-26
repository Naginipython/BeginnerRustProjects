//created using cargo new modules --lib
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}

        pub fn serve_order() {}

        pub fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please!", meal.toast);
    //cant do this line, seasonal_fruit is private
    //meal.seasonal_fruit = String::from("blueberries");

    //I'm iffy on if this works
    println!("What fruit is it today? {}", meal.show_fruit());

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

}

fn deliver_order() {}

mod back_of_house {
    fn fix_order() {
        cook_order();
        //"super" is similar to using ".." in file system
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        //privated, cant access
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
        pub fn show_fruit(&self) -> &String {
            &(self.seasonal_fruit)
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}