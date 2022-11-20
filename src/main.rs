use std::env;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

fn hash(password: &str) {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt);
    if hash.is_err() {
        println!("Error hashing password: {}", hash.err().unwrap());
        return;
    }
    println!("{}", hash.unwrap());
}

fn verify(password: &str, hash: &str) {
    let parsed_hash = PasswordHash::new(&hash).unwrap();
    let matches = Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok();
    if matches {
        println!("true");
    } else {
        println!("false");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() < 3 {
        eprintln!("usage: {} <hash> <password>", program);
        eprintln!("usage: {} <verify> <password> <hash>", program);
        return;
    }

    if let Some(process) = args.get(1) {
        if args.len() == 3 && process == "hash" {
            let password = &args[2];
            hash(password);
            return;
        }
    
        if args.len() == 4 && process == "verify" {
            let password = &args[2];
            let hash = &args[3];
            verify(password, hash);
            return;
        }
    }

}
