extern crate "rust-crypto" as crypto;

use crypto::digest::Digest;
use settings::Settings;
use util::concat_vec;

static CASCADE_ROUNDS: uint = 16;

pub fn hxor<D:Digest>(sh: &mut D, settings: &Settings, password: &Vec<u8>, content: &Vec<u8>) -> Vec<u8> {
    let init = concat_vec(settings.salt.clone(), password.clone()); 
    let mut cascade: Vec<Vec<u8>> = Vec::from_fn(CASCADE_ROUNDS, |x| hash(&mut *sh, &concat_vec(init.clone(), vec![(x + 1) as u8])));
    let hlen = cascade[0].len();
    let mut result: Vec<u8> = Vec::new();

    for (i, c) in content.as_slice().iter().enumerate() {
        let hmod = i % hlen;

        if hmod == 0 {
            for (i, v) in cascade.clone().iter().enumerate() {
                *cascade.get_mut(i) = hash(&mut *sh, &concat_vec(settings.salt.clone(), v.clone()));
            }
        }

        result.push(cascade.iter().fold(c.clone(), |acc, it| acc ^ it[hmod]));
    }

    result
}

pub fn prefix_ident<D:Digest>(sh: &mut D, settings: &Settings, password: &Vec<u8>) -> Vec<u8> {
    hxor(sh, settings, password, &concat_vec(settings.salt_prefix.clone(), password.clone()))
}

pub fn postfix_ident<D:Digest>(sh: &mut D, settings: &Settings, password: &Vec<u8>) -> Vec<u8> {
    hxor(sh, settings, password, &concat_vec(settings.salt_postfix.clone(), password.clone()))
}

pub fn hash<D:Digest>(sh: &mut D, input: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    //perform hashing
    sh.input_str(input.iter().map(|x| x.clone() as char).collect::<String>().as_slice());
    let hash = sh.result_str();
    let h = hash.as_slice();
    sh.reset();

    //populate result
    let mut last = 0u8;
    let mut i    = 0u32;

    for c in h.chars() {
        let cur = chash_to_u8(c);

        if i % 2 == 0 {
            last = cur;
        } else {
            result.push((last * 16) + cur);
        }

        i += 1;
    }

    result
}

fn chash_to_u8(c: char) -> u8 {
    match c {
        '0'=>0, '1'=>1, '2'=>2, '3'=>3, '4'=>4, '5'=>5, '6'=>6, '7'=>7, '8'=>8, '9'=>9,
        'a'=>10, 'b'=>11, 'c'=>12, 'd'=>13, 'e'=>14, 'f'=>15,
         _ =>fail!("exceeded [0-9][a-f]"),
    }
}

