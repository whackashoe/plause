use crypto::sha2::Sha256;
use settings::Settings;
use block::Block;
use item::Item;


/// Plause contains the core functionality for interacting with plause
#[deriving(Clone,Show)]
pub struct Plause {
    pub settings: Settings,
        items:    Vec<Item>,
    pub content:  Vec<u8>,
}

impl Plause {
    pub fn new(blocksize: u32, salt: &str) -> Plause {
        Plause {
            settings: Settings::new(blocksize, salt),
            items:    Vec::new(),
            content:  Vec::new(),
        }
    }

    /// read data in for later extraction tasks
    pub fn import(&mut self, content: Vec<u8>) {
        self.content = content;
    }

    pub fn set_salt(&mut self, salt: Vec<u8>) {
        self.settings.salt = salt;
    }

    pub fn extract(&mut self, password: &Vec<u8>) -> Vec<u8> {
        let mut sh = box Sha256::new();
        let mut fb = Block::new(&mut *sh, &self.settings, &self.items);
        fb.content = self.content.clone();
        fb.extract(&mut *sh, &self.settings, password)
    }

    pub fn add(&mut self, password: &Vec<u8>, content: &Vec<u8>) {
        if content.len() as u32 > self.settings.blocksize {
            fail!("size of file({}) is bigger than blocksize({})", content.len(), self.settings.blocksize);
        }
        for i in self.items.iter() {
            assert!(password.clone() != i.password, "using the same password for multiple inputs allows an agent to see where datablocks begin/end.. don't do it.");
        }

        self.items.push(Item::new(password, content));
    }

    pub fn gen(&mut self) {
        let mut sh = box Sha256::new();
        
        self.content = Block::new(&mut *sh, &self.settings, &self.items).content;
    }

    pub fn get_passwords(&self) -> Vec<Vec<u8>> {
        let mut result:Vec<Vec<u8>> = Vec::new();

        for i in self.items.iter() {
            result.push(i.password.clone());
        }

        return result;
    }
}
