use side::Side;
use order::Order;
use trade::Trade;
use std::collections::BTreeMap;
use std::collections::VecDeque;

pub struct Book {
    side: Side,
    orders: BTreeMap<String, VecDeque<Order>>,
    size: u64
}

impl Book {
    pub fn new(side: Side) -> Book {
        Book {
            side: side,
            orders: BTreeMap::new(),
            size: 0
        }
    }

    pub fn cross(&mut self, order: Order) -> Option<Vec<Trade>> {
        let trades = match order.side {
            Side::Buy => self.cross_with_buy_order(order),
            Side::Sell => self.cross_with_sell_order(order)
        };

        if trades.is_empty() {
            return None
        } else {
            return Some(trades);
        }
    }

    // Basically, we assume the order price will be gte the sell order price.
    fn cross_with_buy_order(&mut self, buy_order: Order) -> Vec<Trade> {
        let mut trades = Vec::new();
        let mut workable_quantity = buy_order.quantity;

        while workable_quantity > 0.0 {
            let mut should_dequeue_top_order = false;

            {
                let sell_order_maybe = self.top();

                if sell_order_maybe == None {
                    break;
                }

                let mut sell_order = sell_order_maybe.unwrap();

                let spread = buy_order.price - sell_order.price;

                if spread < 0.0 {
                    break;
                }

                // TODO: Dequeue an order, or add the ability to adjust it.

                let price = sell_order.price;
                let quantity = buy_order.quantity.max(sell_order.quantity);

                trades.push(Trade {
                    buyer: buy_order.creator.clone(),
                    seller: sell_order.creator.clone(),
                    quantity: quantity,
                    price: price
                });

                sell_order.quantity -= quantity;

                workable_quantity -= quantity;

                if sell_order.quantity == 0.0 {
                    should_dequeue_top_order = true;
                }
            }

            // Apparently I need to do this over here because I need
            // to use brackets to scope the borrowing of self.
            if should_dequeue_top_order {
                self.size -= 1;
                self.dequeue_top();
            }
        }

        return trades;
    }

    fn cross_with_sell_order(&self, order: Order) -> Vec<Trade> {
        return Vec::new();
    }

    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

    pub fn top(&mut self) -> Option<&mut Order> {
        let maybe_price_level = match self.side {
            // Buy for the lowest price possible.
            Side::Buy => self.orders.iter_mut().next(),
            // Sell for the highest price possible.
            Side::Sell => self.orders.iter_mut().last()
        };

        return match maybe_price_level {
            Some((_price_level_key, price_level)) => price_level.front_mut(),
            None => None
        };
    }

    pub fn dequeue_top(&mut self) -> Option<Order> {
        // If we can pop an order, we might need to do some other
        // stuff with more ownership, so splitting the methods is a
        // good logical separation, and it makes the compiler happy.
        let mut popped_order: Option<Order> = self.try_pop_order();

        if popped_order.is_some() {
            let order = popped_order.unwrap();
            let mut price_level_key = order.price_level_key();

            if self.orders.get(&price_level_key).unwrap().is_empty() {
                self.orders.remove(&price_level_key);
            }
        }

        return popped_order;
    }

    fn try_pop_order(&mut self) -> Option<Order> {
        let maybe_price_level = match self.side {
            // Buy for the lowest price possible.
            Side::Buy => self.orders.iter_mut().next(),
            // Sell for the highest price possible.
            Side::Sell => self.orders.iter_mut().last()
        };

        match maybe_price_level {
            Some((_price_level_key, price_level)) => price_level.pop_front(),
            None => None
        }
    }

    pub fn insert(&mut self, order: Order) {
        let price_key = order.price_level_key();

        if !self.orders.contains_key(&price_key) {
            self.orders.insert(price_key, VecDeque::new());
        }

        let a_price_key = order.price_level_key();
        let mut price_level = self.orders.get_mut(&a_price_key).unwrap();
        price_level.push_back(order);

        self.size += 1;
    }

    pub fn size(&self) -> u64 { self.size }
}

#[cfg(test)]
mod test {
    use super::Book;
    use side::Side;
    use order::Order;
    use user::User;

    fn user() -> User { User::new("USR-123") }

    fn order(price: f64, side: Side) -> Order {
        Order {
            price: price,
            quantity: 3.00,
            side: side,
            creator: user()
        }
    }

    #[test]
    fn can_insert_an_order() {
        let order = order(10.00, Side::Buy);
        let mut book = Book::new(Side::Buy);
        assert!(book.is_empty());

        book.insert(order);
        assert!(!book.is_empty());

        let first_order = book.top().unwrap().clone();
        assert_eq!(first_order, order);
    }

    #[test]
    fn can_get_correct_top_order_when_a_sell_book() {
        let order1 = order(10.00, Side::Buy);
        let order2 = order(11.00, Side::Buy);
        let mut book = Book::new(Side::Sell);

        book.insert(order1);
        book.insert(order2);

        let highest_order = book.top().unwrap().clone();
        assert_eq!(highest_order, order2);
    }

    #[test]
    fn can_get_correct_top_order_when_a_buy_book() {
        let order1 = order(10.00, Side::Buy);
        let order2 = order(11.00, Side::Buy);
        let mut book = Book::new(Side::Buy);

        book.insert(order1);
        book.insert(order2);

        let lowest_order = book.top().unwrap().clone();
        assert_eq!(lowest_order, order1);
    }

    #[test]
    fn can_cross_a_sell_book_with_a_buy_order() {
        let mut book = Book::new(Side::Sell);
        let order1 = order(10.00, Side::Sell);
        let order2 = order(11.00, Side::Buy);

        book.insert(order1);
        assert_eq!(book.size(), 1);
        assert_eq!(book.orders.len(), 1);

        book.cross(order2);
        assert_eq!(book.size(), 0);
        assert_eq!(book.orders.len(), 0);
    }
}
