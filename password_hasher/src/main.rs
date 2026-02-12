use std::env;
use std::error::Error;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let password = &args[1];
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|hash_error| format!("argon2 hash error: {hash_error}"))?
        .to_string();


    println!("{}", password_hash);
    Ok(())
}
