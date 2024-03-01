Warning: Project only works on Local Area Network (LAN).

The first file "main.rs" sends message to a desired server. Multiple clients can be used as the same time. 

Here are the dependencies for the client "main.rs":


[dependencies]

tokio = { version = "1", features = ["full"] }

druid = "0.7.0"

druid-derive = "0.5.1"


In the second file "server.rs" is used as the hosting server, it recieves the message and resends it back to all the clients so they can see it. 

Here are the dependencies for the server "server.rs":


[dependencies]

tokio = { version = "1", features = ["full"] }


Project for WAN (Wide Area Network) are comming soon in the future.
