extern crate "rust-crypto" as crypto;

use crypto::digest::Digest;
use std::rand::Rng;
use std::rand;
use settings::Settings;
use item::Item;
use hash::prefix_ident;
use hash::postfix_ident;
use hash::hxor;
use util::check_overlap;

static SEARCH_TIMEOUT_ROUNDS: uint = 16;

#[deriving(Clone,Show)]
pub struct Piece {
        password:  Vec<u8>,
    pub content:   Vec<u8>,
    pub start_pos: u32,
    pub end_pos:   u32,
}

impl Piece {
    pub fn new<D:Digest>(sh: &mut D, settings: &Settings, item: &Item, files: &Vec<Piece>) -> Piece {
        let result: Vec<u8> = Vec::new()
            + prefix_ident(sh, settings, &item.password)
            + hxor(sh, settings, &item.password, &item.content)
            + postfix_ident(sh, settings, &item.password);
        
        let result_len: u32 = result.len() as u32;
        let start_pos:  u32 = find_insert_pos(settings, result_len, files);
        let end_pos:    u32 = start_pos + result_len;

        Piece {
            password:  item.password.clone(),
            content:   result,
            start_pos: start_pos,
            end_pos:   end_pos,
        }
    }
}

fn find_insert_pos(settings: &Settings, result_len: u32, files: &Vec<Piece>) -> u32 {
    for _counter in range(0u, SEARCH_TIMEOUT_ROUNDS) {
        let start_pos: u32 = rand::task_rng().gen_range(0, settings.blocksize);

        match check_overlap(settings, start_pos, result_len, files) {
            true  => { },
            false => { return start_pos; },
        }
    }

    fail!("couldn't find non overlapping segment, try increasing blocksize");
}

