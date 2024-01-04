Warning: Project only works on Local Area Network (LAN).

The first script sends message to a desired server. Multiple clients can be used as teh same time. 

Here are the dependencies for the client:


[dependencies]

tokio = { version = "1", features = ["full"] }

druid = "0.7.0"

druid-derive = "0.5.1"


In the second script that is used as the hosting server, it recieves the message and sends it back to all the clients so they can see it. 

Here are the dependencies for the server:


[dependencies]

tokio = { version = "1", features = ["full"] }


Project for WAN (Wide Area Network) are comming soon in the future.
