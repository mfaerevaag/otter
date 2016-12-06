extern crate ws;
extern crate openssl;
extern crate rustc_serialize;

// use std::{io, str};
// use openssl::pkey::PKey;
// use openssl::rsa::{Rsa, PKCS1_PADDING};
// use rustc_serialize::base64::{self, ToBase64, FromBase64};
// use rustc_serialize::hex::FromHex;

#[macro_use]
mod utils;
mod engine;
mod socket;

// use utils::hexdump;
use engine::Engine;

fn main () {
    // Example demonstration of an encrypted message
    // - encrypt
    // - convert to base64
    // - sent
    // - convert back from base64
    // - decrypt

    // let mut key = PKey::from_rsa(Rsa::generate(2048).unwrap()).unwrap();
    // let mut rsa = key.rsa().unwrap();

    // let s = String::from("SUCH S3CRET");
    // let message = s.as_bytes();
    // let mut cipher: [u8; 256] = [0; 256];
    // let mut result: [u8; 256] = [0; 256];

    // let mut stdout = io::stdout();

    // // display the private key.
    // // println!("public key:");
    // // hexdump(&mut stdout, &rsa.private_key_to_pem().unwrap());

    // println!("message: {}", String::from_utf8_lossy(message));
    // println!("bytes[{}]: {:?}", &message.len(), &message);

    // // encrypt the string and display the ciphertext.
    // let res = rsa.public_encrypt(&message, &mut cipher, PKCS1_PADDING).unwrap();
    // println!("cipher: [{}]", res);
    // hexdump(&mut stdout, &cipher);

    // let base = cipher.to_base64(base64::STANDARD);

    // println!("sent message[{}]: {:?}\n", base.as_bytes().len(), &base.as_bytes());

    // let s = base.from_base64().unwrap();

    // // Decrypt the string and show the plaintext.
    // let res = rsa.private_decrypt(&s, &mut result, PKCS1_PADDING).unwrap();
    // println!("decrypted (hex):");
    // hexdump(&mut stdout, &result);
    // println!("decrypted (string): \"{}\"", str::from_utf8(&result).unwrap());


    let engine = Engine::new("ws://127.0.0.1:10000");

    engine.run();
}
