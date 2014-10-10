extern crate "rust-crypto" as crypto;

use crypto::digest::Digest;
use std::rand::Rng;
use std::rand;
use settings::Settings;
use item::Item;
use piece::Piece;
use hash::prefix_ident;
use hash::postfix_ident;
use hash::hxor;
use util::find_file;
use util::find_file_next_distance;

#[deriving(Show)]
pub struct Block {
        files:     Vec<Piece>,
    pub content:   Vec<u8>,
}

impl Block {
    pub fn new<D:Digest>(sh: &mut D, settings: &Settings, items: &Vec<Item>) -> Block {
        let mut result: Vec<u8> = Vec::with_capacity(settings.blocksize as uint);
        result.grow(settings.blocksize as uint, 0u8);
        let mut files: Vec<Piece> = Vec::new();

        for i in items.iter() {
            let oldfiles = files.clone();
            files.push(Piece::new(sh, settings, i, &oldfiles));
        }

        //apply our files to the buffer
        let mut i: uint = 0;
        
        loop {
            let dist = find_file_next_distance(settings, i as u32, files.to_vec());

            match dist {
                Some(0) => {
                    let file = find_file(i as u32, files.to_vec());

                    match file {
                        Some(x) => {
                            for m in x.content.iter() {
                                *result.get_mut(i) = m.clone();
                                i += 1;
                            }
                        },
                        None => {
                            fail!("expected file, found shit");
                        }
                    }
                },
                Some(x) => {
                    for _pos in range(0, x) {
                        *result.get_mut(i) = Block::rand_byte();
                        i += 1;
                    }
                },
                None => {
                    //end of road (no more files left)
                    for _pos in range(0, settings.blocksize - (i as u32)) {
                        *result.get_mut(i) = Block::rand_byte();
                    }
                    break;
                },
            }
        }

        Block {
            files: files,
            content: result,
        }
    }

    fn rand_byte() -> u8 {
        rand::task_rng().gen_range(0u8, 255)
    }

    fn find_needle_pos(&self, needle: &Vec<u8>) -> Option<uint> {
        for (i, vi) in self.content.iter().enumerate() {
            let first_letter = needle[0];

            if vi == &first_letter {
                let mut success: bool = true;

                for (j, vj) in needle.iter().enumerate() {
                    if &self.content[i + j] != vj {
                        success = false;
                        break;
                    }
                }

                if success {
                    return Some(i);
                }
            }
        }

        None
    }

    fn find_needle_endpos(&self, needle: &Vec<u8>) -> Option<uint> {
        match self.find_needle_pos(needle) {
            Some(x) => Some(x + needle.len()),
            None    => None,
        }
    }

    pub fn extract<D:Digest>(&self, sh: &mut D, settings: &Settings, password: &Vec<u8>) -> Vec<u8> {
        let range_end   = self.find_needle_endpos(&prefix_ident(sh, settings, password));
        let range_begin = self.find_needle_pos(  &postfix_ident(sh, settings, password));

        let msg = match range_end {
            Some(begin) => match range_begin {
                Some(end) => {
                    self.content[begin..end].to_vec()
                },
                None    => fail!("No postfix needle found")
            },
            None    => fail!("No prefix needle found")
        };

        hxor(sh, settings, password, &msg)
    }
}
