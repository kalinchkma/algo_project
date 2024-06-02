use tokio::net::{TcpListener, TcpStream};
use mini_redis::{ Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};


const PORT: u32 = 6379;
const HOST: &str = "127.0.0.1";

#[tokio::main]
async fn main() {
    
    // bind listener to the address
    let listener = TcpListener::bind(format!("{}:{}", HOST, PORT)).await.unwrap();

    // log server running
    println!("Listening to the port: {}", PORT);
    println!("Host: http://{}:{}", HOST, PORT);

    // creatin hash map db to share the stated
    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // the second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();

        // clone the handle to the hash map
        let db = db.clone();

        tokio::spawn(async move {
            process(socket, db).await;
        });

    }
}

type ShardedDb = Arc<Vec<Mutex<HashMap<String,Vec<u8>>>>>;

async fn new_shared_db(num_shards: usize) -> ShardedDb {
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()))
    }
    Arc::new(db)
}


async fn process(socket: TcpStream, db: Arc<Mutex<HashMap<String, Vec<u8>>>>) {
    use mini_redis::Command::{self, Get, Set};


    // A hashmap is used to store data
    // let mut db = HashMap::new();

    // The `Connection` lets us read/write redis **frames** instead of 
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);


    // Use `read_frame` to receive a command from the connection
    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // the value is stored as `Vec<u8>`
                // lock before using the db
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("Ok".to_string())
            },
            Get(cmd) => {
                // lock the mutex before using it
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            },
            cmd => panic!("unimplemented! {:?}", cmd)
        };

        // // Response with an error
        // let response = Frame::Error("unimplemented".to_string());

        connection.write_frame(&response).await.unwrap();
    }

}
