use side::Side;
use user::User;

use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Order {
    pub price: f64,
    pub quantity: f64,
    pub side: Side,
    pub creator: User,
}

impl Order {
    pub fn price_level_key(&self) -> String {
        self.price.to_string()
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CREATOR: {0}, SIDE: {1}, PRICE: {2}, QUANTITY: {3}",
               self.creator, self.side, self.price, self.quantity)
    }
}

#[cfg(test)]
mod test {
    use side::Side;
    use user::User;
    use super::Order;

    fn user() -> User { User::new("USR-123") }

    fn subject(price: f64) -> Order {
        Order {
            price: price,
            quantity: 3.00,
            side: Side::Buy,
            creator: user()
        }
    }

    #[test]
    fn create_an_order() {
        let order = subject(100.00);

        // Not a good test, but shows we can make it.
        assert!(true);
    }

    #[test]
    fn it_can_make_a_string_of_the_price() {
        let order = subject(10.00);
        assert_eq!(order.price_level_key(), "10");

        let order2 = subject(10.50);
        assert_eq!(order2.price_level_key(), "10.5");
    }
}
