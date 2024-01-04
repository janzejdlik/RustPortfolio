use tokio::net::UdpSocket;
use std::collections::HashSet;
use std::error::Error;
use std::str;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Bind the server to a specific port
    let socket = UdpSocket::bind("192.168.1.198:12345").await?;
    println!("Server listening on port 12345");

    // HashSet to store the addresses of the clients
    let mut clients = HashSet::new();

    loop {
        let mut buf = [0u8; 1024];
        // Receive messages
        match socket.recv_from(&mut buf).await {
            Ok((len, addr)) => {
                clients.insert(addr); // Add new client address
                let msg = str::from_utf8(&buf[..len])?;

                // Display the received message on the server
                println!("Received from {}: {}", addr, msg);

                // Broadcast the message to all clients
                for client in &clients {
                    if *client != addr { // Avoid sending the message back to the sender
                        socket.send_to(&buf[..len], client).await?;
                    }
                }
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
