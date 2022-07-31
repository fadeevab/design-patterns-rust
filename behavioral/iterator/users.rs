pub struct UserCollection {
    users: [&'static str; 3],
}

impl UserCollection {
    pub fn new() -> Self {
        Self {
            users: ["Alice", "Bob", "Carl"],
        }
    }

    pub fn iter(&self) -> UserIterator {
        UserIterator {
            index: 0,
            user_collection: self,
        }
    }
}

/// UserIterator allows sequential traversal through a complex users collection
/// without exposing its internal details.
pub struct UserIterator<'a> {
    index: usize,
    user_collection: &'a UserCollection,
}

// `Iterator` is a standard interface for dealing with iterators from the Rust standard library.
impl Iterator for UserIterator<'_> {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.user_collection.users.len() {
            let user = Some(self.user_collection.users[self.index]);
            self.index += 1;
            return user;
        }

        None
    }
}
