use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct User {
    guid: &'static str,
}

impl User {
    pub fn new(guid: &'static str) -> User {
        User { guid: guid }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(guid: {})", self.guid)
    }
}

#[cfg(test)]
mod test {
    use super::User;

    #[test]
    fn it_can_create_a_user_with_a_guid() {
        let user = User { guid: "test" };
        assert_eq!(user.guid, "test");
    }
}
