use mini_redis::{Result, client};


#[tokio::main]
pub async fn main() -> Result<()> {
    // open a connection to mini-redis addr
    let mut client = client::connect("127.0.0.1:6379").await?;

    // set "world" value to "hello" key
    client.set("hello", "world".into()).await?;

    // get "hello" value
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
