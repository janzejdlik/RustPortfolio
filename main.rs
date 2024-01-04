use tokio::net::UdpSocket;
use tokio::task;
use std::error::Error;
use std::io::{self, Write};
use std::str;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_addr = "192.168.1.185:12345";
    let send_socket = UdpSocket::bind("0.0.0.0:0").await?;
    send_socket.connect(server_addr).await?;

    let recv_socket = UdpSocket::bind("0.0.0.0:0").await?;
    recv_socket.connect(server_addr).await?;

    // Start a task for receiving messages
    let recv_task = task::spawn(async move {
        let mut buf = [0; 1024];
        loop {
            match recv_socket.recv(&mut buf).await {
                Ok(num_bytes) => {
                    println!("Received: {}",
str::from_utf8(&buf[..num_bytes]).expect("Invalid UTF-8"));
                },
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    });

    // Main loop for sending messages
    loop {
        print!("Enter message: ");
        io::stdout().flush().unwrap();

        let mut message = String::new();
        io::stdin().read_line(&mut message)?;
        let message = message.trim_end();

        if message.eq_ignore_ascii_case("exit") {
            break;
        }

        send_socket.send(message.as_bytes()).await?;
    }

    recv_task.await?;
    Ok(())
}