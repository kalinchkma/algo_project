use std::time;

#[allow(dead_code)]
use tokio::task;
use reqwest::Error;


async fn send_request(client: &reqwest::Client, url: &str) -> Result<(), Error> {

     match client.get(url).send().await {
        Ok(c) => {
            print!("{:?}\n", c.status() );
        },
        Err(e) => {
            return Err(e)
        }
    };
    Ok(())
}  

async fn send_concurrent_request(url: &str, num_of_req: usize) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let mut tasks = Vec::new();

    for _ in 0..num_of_req {
        let client = client.clone();
        let url = url.to_string();
        let task = task::spawn(async move  {
            if let Err(e) = send_request(&client, &url).await {
                eprintln!("Request failed: {}", e)
            }
        });
        tasks.push(task);
    }

    for task in tasks {
        match task.await {
            Ok(_) => (),
            Err(_) => ()
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let url = "http://103.213.38.9:31337/register";
    let num_request = 1000000;
    let start = time::Instant::now();
    match send_concurrent_request(&url, num_request).await {
        Ok(_) => println!("\nAll request have been sent"),
        Err(e) => println!("\nError: {}", e)
    }  

    let stop = start.elapsed();

    println!("Time taken of request: {:?}", stop);
   
}
