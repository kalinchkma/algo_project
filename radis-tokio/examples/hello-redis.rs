use mini_redis::{client, Result};

async fn say_world() {
    print!("world\n");
}

#[tokio::main]
async fn main() -> Result<()> {
    // open a connection to the mini-redis address
    let mut client = client::connect("127.0.0.1:6379").await?;

    // set the key 'hello' with value 'world'
    client.set("hello", "world".into()).await?;

    let op = say_world();
    // Get key "hello"
    let result = client.get("hello").await?;

    println!("Got value from server; result={:?}", result.unwrap());

    print!("Hello ");
    op.await;


    Ok(())
}


