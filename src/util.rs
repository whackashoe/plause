use std::rand;
use std::rand::Rng;
use settings::Settings;
use piece::Piece;

pub fn check_overlap(settings: &Settings, pos: u32, content_len: u32, files: &Vec<Piece>) -> bool {
    for i in files.iter() {
        if pos + 1 > i.start_pos               && pos - 1 < i.end_pos
        || pos + 1 + content_len > i.start_pos && pos - 1 + content_len < i.end_pos
        || pos + 1 + content_len > settings.blocksize - 1 {
            return true;
        }
    }

    false
}

pub fn find_file_next_distance(settings: &Settings, pos:u32, files:Vec<Piece>) -> Option<u32> {
    let mut dist: u32 = settings.blocksize - pos;
    let mut found: bool = false;

    for i in files.iter() {
        if pos > i.start_pos && pos < i.end_pos {
            return Some(0u32);
        } else {
            let spos = i.start_pos - pos;

            if spos < dist {
                dist = spos;
                found = true;
            }
        }
    }

    return if found { Some(dist) } else { None };
}

pub fn find_file(pos: u32, files: Vec<Piece>) -> Option<Piece> {
    for i in files.iter() {
        if pos >= i.start_pos && pos < i.end_pos {
            return Some(i.clone());
        }
    }

    None
}

pub fn concat_vec<T>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut r = a;
    r.extend(b.into_iter());
    r
}

pub fn random_pass(len: uint) -> Vec<u8> {
    rand::task_rng().gen_ascii_chars().take(len).map(|x| x as u8).collect::<Vec<u8>>()
}

