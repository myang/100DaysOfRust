mod front_of_house;
mod back_of_house;

pub use crate::front_of_house::hosting;
pub use crate::front_of_house::serving;
use crate::back_of_house::breakfast;

pub fn eat_at_restaurant() {
    // relative path
    hosting::add_to_waitlist();

    let mut meal = breakfast::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
