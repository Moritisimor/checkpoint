# Checkpoint
An easy-to-use, easily deployable and lightweight Reverse Proxy written in Rust using the Actix-Web Framework 

# What is this project?
Checkpoint is designed to be a lightweight and easily usable reverse proxy. It is configurable with a single JSON file and can be deployed in a single binary.

# Why Rust/Actix-Web?
- Rust because it has guaranteed memory- and thread-safety, high performance and because it compiles to small, native binaries.
- Actix-Web because it allows me to make very small and fast servers, while not being too difficult to use.

# What features are planned?
Planned features include, but are not limited to...
- Header Injection
- Logging
- Request-Rate limiting
- Forwarding Bodies

# What features are there already?
Currently, checkpoint can...
- Forward Requests to services
- Forward Responses of services back to the clients
- Block blacklisted IPs
- Append CORS Headers

# How do I compile it?
You will need cargo to compile this.

First, clone the repository:
```bash
git clone https://github.com/Moritisimor/checkpoint
```
Then, cd into the directory where the source code resides:
```bash
cd checkpoint/src
```
And finally compile it:
```bash
cargo build -r
```
You can find the compiled binary in ```checkpoint/target/release```

# How do I use it?
To use Checkpoint, all you need to do is write a config JSON. An example of such a config file is in the root folder of this repository.

Checkpoint will check its own working directory for this file and expects it to be named ```config.json```, anything else won't work.

When Checkpoint is running, it will check its internal hashmap, which stores services and their URLs, and get the URL of the service the client entered.

The service is always the first HTTP Parameter, so for example, in a request like ```GET http://localhost:9000/MyService/ping```, the MyService part is the service.

Internally, Checkpoint will then construct a URL to send the request to. In this example, if ```MyService``` were to point to ```http://localhost:3000```, it would cut off the first parameter, so the URI would just be ```/ping``` and replace its own address, which the client sent a request to, with the service's, so it would become ```http://localhost:3000/ping```

After that, Checkpoint will await the service's response, and send it back to the client.
