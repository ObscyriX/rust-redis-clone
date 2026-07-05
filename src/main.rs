use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

mod resp;
mod resp_result;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Server Started!");

    // Create TCP Server
    // Redis Port - 6379
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    // Incoming Request Process in stream
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(handle_listener(stream));
            }

            Err(e) => {
                println!("Error {}", e);
                continue;
            }
        }
    }
}

async fn handle_listener(mut stream: TcpStream) {
    println!("Connection Accepted");
    // create a buffer
    let mut buffer = [0; 512];

    loop {
        //Read from the strream into buffer
        match stream.read(&mut buffer).await {
            // if stream return some data
            Ok(size) if size != 0 => {
                let response = "+PONG\r\n";

                if let Err(e) = stream.write_all(response.as_bytes()).await {
                    eprintln!("Error in reading stream: {}", e);
                }
            }
            Ok(_) => {
                println!("Conection closed");
                break;
            }
            Err(e) => {
                println!("Error {}", e);
                break;
            }
        }
    }
}
