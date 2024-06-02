use std::process;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::fs::File;
use std::env::{args};

struct Crypto {
    key: u32,
}

impl Crypto {
    // Constructor
    fn new(key: u32) -> Crypto {
        Crypto {
            key
        }
    }

    // encrypt data
    fn encrypt(&self, vec: &Vec<u8>) -> Vec<u8> {
        let mut hash_vec = Vec::new();

        for n in vec.iter() {
            // calculate encryption
            let n = *n + (self.key as u8);
            hash_vec.push(n);
        }

        hash_vec
    }

    // decrypt data
    fn decrypt(&self, vec: &Vec<u8>) -> Vec<u8>{
        let mut data = Vec::new();

        for n in vec.iter() {
            // calculate decryption
            let n = *n - (self.key as u8);
            data.push(n);
        }

        data
    }

}

// testing module of crypto struct
#[cfg(test)]
mod test_crypto {
    use super::*;

    #[test]
    fn test_encryption() {
        let crypto = Crypto::new(10);

        let vec: Vec<u8> = vec![1, 2, 3, 4, 5];
        let target: Vec<u8> = vec![11, 12, 13, 14, 15];

        assert_eq!(crypto.encrypt(&vec), target);

        assert_eq!(crypto.decrypt(&target), vec);
    }
}

#[derive(Debug)]
enum Flg {
    Encrypt,
    Decrypt,
    Nil
}

impl Flg {
    fn new(flg: String) -> Flg {
        if flg == "-e" {
            Flg::Encrypt
        } else if flg == "-d" {
            Flg::Decrypt
        } else {
            Flg::Nil
        }
    }
}

#[derive(Debug)]
struct ArgumentParser {
    flg: Flg,
    input_file: String,
    output_file: String
}

impl ArgumentParser {
    // constructor
    fn new(arguments: Vec<String>) -> ArgumentParser {

        // check the arguemt 
        if  arguments.len() <= 1 {
            println!("Not Enough Arguments");
            process::exit(0)
        }

        if arguments.len() > 4 {
            println!("too many arguments");
            println!("Useage:\n\t -e encrypt file\n\t-d decrypt file\n\t--h or --help for help menu");
            process::exit(0)
        } 

        else if arguments.len() == 2 && (arguments[1] == "--h" || arguments[1] == "--help") {
            println!("Useage:\n\t-e encrypt file\n\t-d decrypt file\n\t--h or --help for help menu");
            println!("Example:\n\t-e normal_file encrypted_file\n\t-d encrypted_file decrypted_file");
            process::exit(0)
        } else if arguments.len() == 2 && (arguments[1] != "--h" || arguments[1] != "--help") {
            println!("invalid arguments");
            println!("Useage:\n\t -e encrypt file\n\t-d decrypt file\n\t--h or --help for help menu");
            process::exit(0)
        }

        let flg = Flg::new(arguments[1].clone());
        let input_file = arguments[2].clone();
        let output_file = arguments[3].clone();

        ArgumentParser {
            flg,
            input_file,
            output_file
        }
    }
}



#[tokio::main]
async fn main() {
    let args: Vec<String> = args().collect();

    let args = ArgumentParser::new(args);
    let crypto = Crypto::new(90);
    
    match args.flg {
        Flg::Encrypt => {
            let  file_content = match read_file(&args.input_file).await {
                Ok(c) => c,
                Err(e) => {
                    println!("Error: {}", e);
                    process::exit(0)
                }
            };
            let encrypt_data = crypto.encrypt(&file_content);
            match write_file_with_u8(&args.output_file, encrypt_data).await {
                Ok(_) => println!("File encrypted successfully: {}", args.output_file),
                Err(e) => println!("Failed to encrypt the file {}", e)
            }
        },
        Flg::Decrypt => {
            let file_content = match read_file(&args.input_file).await {
                Ok(c) => c,
                Err(e) => {
                    println!("Error: {}", e);
                    process::exit(0)
                }
            };
            let decrypt_data = crypto.decrypt(&file_content);

            match write_file_with_u8(&args.output_file, decrypt_data).await {
                Ok(_) => println!("File decrypted successfully: {}", args.output_file),
                Err(e) => println!("Error: {}", e)
            }
        },
        Flg::Nil => {
            println!("invalid arguments");
            println!("Useage:\n\t -e encrypt file\n\t-d decrypt file\n\t--h or --help for help menu");
            process::exit(0)
        }
    }
}



async fn read_file(path: &str) -> Result<Vec<u8>, io::Error> {
    let mut file = match File::open(path).await {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut buffer = Vec::new();

    match file.read_to_end(&mut buffer).await {
        Ok(_) => (),
        Err(e) => return  Err(e),
    };

    return  Ok(buffer);
}

async fn write_file_with_u8(path: &str, contents: Vec<u8>) -> Result<(), io::Error> {
    let mut file = match File::create(path).await {
        Ok(f) => f,
        Err(e) => return Err(e)
    };

    for c in contents.iter() {
        file.write_u8(*c).await?
    }

    return Ok(());
}