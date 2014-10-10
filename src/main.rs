#![crate_type="bin"]
#![feature(slicing_syntax)]

extern crate "rust-crypto" as crypto;
extern crate getopts;

use std::os;
use std::from_str::FromStr;
use std::io;
use std::io::File;
use std::io::BufferedReader;
use getopts::{optopt,optflag,optmulti,getopts,OptGroup};
use util::random_pass;
use plause::Plause;

mod hash;
mod util;
mod item;
mod piece;
mod block;
mod settings;
mod plause;

static VERSION:                             f32 = 0.01;

static DEFAULT_ENCRYPTFILE:        &'static str = "output.enc";
static DEFAULT_DECRYPTFILE:        &'static str = "output.dec";
static DEFAULT_PASSWORDECRYPTFILE: &'static str = "pass.key";
static DEFAULT_SALT:               &'static str = "saltysaltsalt";
static DEFAULT_BLOCKSIZE:                   u32 = 1048576;

static CONT_ENCRYPT_FILE:     &'static str = "E";
static CONT_PASSWORD_FILE:    &'static str = "P";
static CONT_SET_BLOCKSIZE:    &'static str = "b";
static CONT_SET_SALT:         &'static str = "s";
static CONT_HELP_MODE:        &'static str = "h";
static CONT_VERSION_MODE:     &'static str = "v";
static CONT_DECRYPT_MODE:     &'static str = "d";
static CONT_ENCRYPT_MODE:     &'static str = "e";
static CONT_INTERACTIVE_MODE: &'static str = "i";

#[deriving(PartialEq, Eq)]
pub enum Mode {
    Help,
    Version,
    Interactive,
    Encrypt,
    Decrypt,
}

fn main() {
    let args: Vec<String> = os::args();
    let program = args[0].clone();
    let opts = [
        optopt(CONT_ENCRYPT_FILE,  "encrypt-file",  format!("set encryption output file (default: {})", DEFAULT_ENCRYPTFILE).as_slice(), "FILENAME"),
        optopt(CONT_PASSWORD_FILE, "password-file", format!("set password file (default: {})", DEFAULT_PASSWORDECRYPTFILE).as_slice(),   "FILENAME"),
        
        optopt(CONT_SET_BLOCKSIZE, "blocksize", format!("set block size (default: {})", DEFAULT_BLOCKSIZE).as_slice(), "SIZE"),
        optopt(CONT_SET_SALT,      "salt",      format!("set salt (default: {})", DEFAULT_SALT).as_slice(),            "SALT"),

        optflag(CONT_HELP_MODE,        "help",        "print this help menu"),
        optflag(CONT_VERSION_MODE,     "version",     "show version"),
        optflag(CONT_INTERACTIVE_MODE, "interactive", "interactively add text messages"),
        optmulti(CONT_ENCRYPT_MODE,    "encrypt",     "encrypt file(s)", "FILENAME"),
        optopt(CONT_DECRYPT_MODE,      "decrypt",     "decrypt file using password-file and filename prefix for output", "FILENAME"),
    ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m)  => { m },
        Err(f) => { fail!(f.to_string()) },
    };

    let blocksize     = fallback_match_fstr(CONT_SET_BLOCKSIZE, &matches, DEFAULT_BLOCKSIZE);
    let salt          = fallback_match(CONT_SET_SALT,           &matches, DEFAULT_SALT);
    let encrypt_file  = fallback_match(CONT_ENCRYPT_FILE,       &matches, DEFAULT_ENCRYPTFILE);
    let password_file = fallback_match(CONT_PASSWORD_FILE,      &matches, DEFAULT_PASSWORDECRYPTFILE);

    let mut plause = Plause::new(blocksize, salt.as_slice());

    match get_mode(&matches) {
        Help             => print_usage(program.as_slice(), opts),
        Version          => print_version(program.as_slice()),
        Interactive      => encrypt_interactive(&mut plause,
                                                &Path::new(encrypt_file.as_slice()),
                                                &Path::new(password_file.as_slice()),
                                                salt.as_slice()),
        Encrypt          => encrypt_files(&mut plause,
                                          &Path::new(encrypt_file.as_slice()),
                                          &Path::new(password_file.as_slice()),
                                          salt.as_slice(),
                                          &matches.opt_strs(CONT_ENCRYPT_MODE).iter().map(|x| Path::new(x.as_slice())).collect::<Vec<Path>>()),
        Decrypt          => decrypt(&mut plause,
                                    &Path::new(encrypt_file.as_slice()),
                                    &Path::new(matches.opt_str(CONT_DECRYPT_MODE).unwrap_or(String::from_str(DEFAULT_DECRYPTFILE))),
                                    &Path::new(password_file.as_slice())),
    }
}

fn print_usage(program: &str, opts: &[OptGroup]) {
    println!("{}", getopts::short_usage(program.as_slice(), opts));
    println!("plausible deniability for love and friendship");
    println!("{}", getopts::usage("", opts));
    println!("");
    println!("Examples:");
    println!("    ./plause -b 120000 -s secretsaltysalt -eFILE1 -eFILE2");
    println!("\t\t\tencrypt FILE1 and FILE2 into output.enc of 120kb size")
    println!("\t\t\tusing generated passwords secretsaltysalt as salt");
    println!("\t\t\tsaving them into pass.key");
    println!("    ./plause -d output_files");
    println!("\t\t\tDecrypt output.enc to output_files.# with salt/passwords");
    println!("\t\t\tfrom pass.key");
    println!("");
    println!("report all bugs to github.com/whackashoe/plause");
}

fn print_version(program: &str) {
    println!("{} {}", program, VERSION);
}

fn decrypt(plause: &mut Plause, encrypt_path: &Path, decrypt_path: &Path, password_path: &Path) {
    let mut passwords: Vec<Vec<u8>> = Vec::new();
    let mut enough_lines: bool = false;

    match File::open(password_path) {
        Ok(f)  => {
            let mut file = BufferedReader::new(f);
            for (i, line) in file.lines().enumerate() {
                if i == 0 {
                    plause.set_salt(line.unwrap().into_bytes().init().to_vec());
                } else {
                    enough_lines = true;
                    passwords.push(line.unwrap().into_bytes().init().to_vec());
                }
            }

            if !enough_lines {
                fail!("password file requires at least two lines (salt) and (password(s))");
            }
        },
        Err(e) => fail!("decrypt open password path: {}", e),
    }

    match File::open(encrypt_path).read_to_end() {
        Ok(f)  => plause.import(f),
        Err(e) => fail!("decrypt open encrypt path: {}", e),
    }

    for (i, password) in passwords.iter().enumerate() {
        let decpath = if passwords.len() == 1 {
            decrypt_path.clone()
        } else {
            Path::new(format!("{}.{}", decrypt_path.display(), i))
        };
        match File::create(&decpath).write(plause.extract(password).as_slice()) {
            Ok(f)  => f,
            Err(e) => fail!("decrypt create decrypt path: {}", e),
        };
    }
}

fn encrypt_interactive(plause: &mut Plause, encrypt_path: &Path, password_path: &Path, salt: &str) {
    for line in io::stdin().lines() {
        plause.add(&random_pass(24), &line.unwrap().into_bytes().init().to_vec());
    }

    plause.gen();
    write_results(plause, encrypt_path, password_path, salt);
}

fn encrypt_files(plause: &mut Plause, encrypt_path: &Path, password_path: &Path, salt: &str, files: &Vec<Path>) {
    for file in files.iter() {
        let contents = File::open(file).read_to_end();
        match contents {
            Ok(f)  => plause.add(&random_pass(24), &f),
            Err(e) => fail!("error reading file: {}", e),
        }
    }

    plause.gen();
    write_results(plause, encrypt_path, password_path, salt);
}
fn fallback_match(c: &str, matches: &getopts::Matches, default: &str) -> String {
    if matches.opt_present(c) {
        return matches.opt_str(c).unwrap_or(default.to_string());
    }

    default.to_string()
}

fn fallback_match_fstr<T: FromStr>(c: &str, matches: &getopts::Matches, default: T) -> T {
    if matches.opt_present(c) {
        match matches.opt_str(c) {
            Some(y) => {
                return match from_str(y.as_slice()) {
                    Some(x) => x,
                    None    => default,
                };
            },
            None    => { return default; }
        };
    }

    default
}

fn get_mode(matches: &getopts::Matches) -> Mode {
    if matches.opt_present(CONT_HELP_MODE) {
        return Help;
    }

    if matches.opt_present(CONT_VERSION_MODE) {
        return Version;
    }
    
    if matches.opt_present(CONT_INTERACTIVE_MODE) {
        return Interactive;
    }

    if matches.opt_present(CONT_ENCRYPT_MODE) {
        return Encrypt;
    }

    if matches.opt_present(CONT_DECRYPT_MODE) {
        return Decrypt;
    }

    Help
}

fn write_results(plause: &mut Plause, encrypt_file: &Path, password_file: &Path, salt: &str) {
    match File::create(encrypt_file).write(plause.content.as_slice()) {
        Ok(f)  => f,
        Err(e) => fail!("encrypt file error: {}", e),
    };

    let mut pfile = match File::create(password_file) {
        Ok(f)  => f,
        Err(e) => fail!("password file error: {}", e),
    };

    pfile.write_line(salt.as_slice()).unwrap();
    for i in plause.get_passwords().iter() {
        pfile.write(i.as_slice()).unwrap();
        pfile.write_line("").unwrap();
    }
}

