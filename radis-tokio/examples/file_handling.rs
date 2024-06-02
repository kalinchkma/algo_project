

use std::process;

use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("article_in_number.txt").await.unwrap_or_else(|err| {
        println!("Error reading file:\n{}", err);
        process::exit(0)
    });

    let mut buffer: Vec<u8> = Vec::new();

    // read upto 10 bytes
    let _n = f.read_to_end(&mut buffer).await.unwrap_or_else(|err| {
        panic!("Error reading: \n{}", err);
    });


    // println!("The byte buffer: {:?}", &buffer[..n]);
    println!("The byte buffer:\n{:?}", &buffer);
    
    println!("Lenght of the file:\n{}", buffer.len());

    // make byte vector to binary vector
    let byto_to_binary: Vec<String> = buffer.iter().map(|&byte| {
        // (0..8).rev().map(move |i| ((byte >> i) & 1) as u8).collect::<Vec<u8>>();
        format!("{:08b}", byte.clone())
    }).collect();

    println!("Content in binary {:?}", byto_to_binary);
    // match std::str::from_utf8(&buffer[.._n]) {
    //     Ok(s) => println!("The character, {}", s),
    //     Err(e) => println!("Invalid utf-8 sequence {}", e)
    // }

    // write to a file file
    let mut file = File::create("article_in_number.txt").await.unwrap_or_else(|err| {
        println!("Error creating file: {}", err);
        process::exit(0)
    });

    // write number buffer into file
    for n in buffer.iter() {
        file.write_u8(*n+2).await?;
    }

    Ok(())
}


