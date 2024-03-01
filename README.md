Warning: Project only works on Local Area Network (LAN).

<br/>

<h2>Client</h2>


Download Rust from [Rust Official Website](https://www.rust-lang.org/tools/install). Sometimes cargo install is required as well.
Once Rust and Cargo is downloaded, create new Rust project directory with cargo:

```console
cargo new project_name
```
Then navigate to project directory by typing:

```console
cd project_name/src
```
Replace the content of your "main.rs" file with this project's "main.rs" file.


File "main.rs" sends message to a desired server. Multiple clients can be used as the same time. 

Here are the dependencies for the client "main.rs":

```toml
[dependencies]

tokio = { version = "1", features = ["full"] }

druid = "0.7.0"

druid-derive = "0.5.1"
```
Now client-side application is finished.


<h2>Server</h2>


The same process should be repeated for the server-side of the application

In the second file "server.rs" is used as the hosting server, it recieves the message and resends it back to all the clients so they can see it. 

Here are the dependencies for the server "server.rs":
```toml
[dependencies]

tokio = { version = "1", features = ["full"] }
```


Project for WAN (Wide Area Network) are comming soon in the future.
