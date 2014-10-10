#[deriving(Clone,Show)]
pub struct Item {
    pub password:  Vec<u8>,
    pub content:   Vec<u8>,
}

impl Item {
    pub fn new(password: &Vec<u8>, content: &Vec<u8>) -> Item {
        Item {
            password: password.clone(),
            content:  content.clone(),
        }
    }
}
