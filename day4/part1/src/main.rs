
use rand::Rng;
use std::env;


fn main() {
    let length:usize = env::args()
        .nth(1)
        .unwrap_or("12".to_string())
        .parse()
        .unwrap();


    let charset = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";


    let mut rng = rand::thread_rng();
    let password:String = (0..length)
        .map(|_|{
            let index  = rng.gen_range(0..charset.len());
            charset[index] as char
        })
        .collect();

    println!("Generated {}", password);
}