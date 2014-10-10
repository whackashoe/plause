static DEFAULT_PREFIX_SALT:  &'static str = "@7@llc05754261933$uFf3r";
static DEFAULT_POSTFIX_SALT: &'static str = "11419523LIMin@73pRiv@cY";

#[deriving(Clone,Show)]
pub struct Settings {
    pub blocksize:    u32,
    pub salt:         Vec<u8>,
    pub salt_prefix:  Vec<u8>,
    pub salt_postfix: Vec<u8>,
}

impl Settings {
    pub fn new(blocksize:u32, salt:&str) -> Settings {
        Settings {
            blocksize:    blocksize,
            salt:         String::from_str(salt).into_bytes(),
            salt_prefix:  format!("{}{}", DEFAULT_PREFIX_SALT,  salt).into_bytes(),
            salt_postfix: format!("{}{}", DEFAULT_POSTFIX_SALT, salt).into_bytes(),
        }
    }
}
