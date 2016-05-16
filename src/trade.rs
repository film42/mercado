use user::User;

pub struct Trade {
    pub buyer: User,
    pub seller: User,
    pub quantity: f64,
    pub price: f64
}
