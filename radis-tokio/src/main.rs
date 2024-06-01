use tokio::net::{TcpListener, TcpStream};
use mini_redis::{cmd::{Get, Set}, Connection, Frame};


#[tokio::main]
async fn main() {
    // bind listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // the second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            process(socket).await;
        });

    }
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};

    use std::collections::HashMap;

    // A hashmap is used to store data
    let mut db = HashMap::new();

    // The `Connection` lets us read/write redis **frames** instead of 
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);


    // Use `read_frame` to receive a command from the connection
    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // the value is stored as `Vec<u8>`
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("Ok".to_string())
            },
            Get(cmd) => {
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
