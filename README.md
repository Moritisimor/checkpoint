# Checkpoint
A easy-to-use, easily deployable and lightweight Reverse Proxy written in Rust using the Actix-Web Framework 

# What is this project?
Checkpoint is designed to be a lightweight and easily usable reverse proxy. It is configurable with a single JSON file and can be deployed in a single binary.

# Why Rust/Actix-Web?
- Rust because it has guaranteed memory- and thread-safety, high performance and because it compiles to small, native binaries.
- Actix-Web because it allows me to make very small and fast servers, while not being too difficult to use.

# What features are planned?
This project is still highly WIP, and as such, not many features are implemented yet, but planned features include, but are not limited to...
- Mapping external services to proxy-endpoints
- Blacklists/Blocking IPs
- CORS
- Header Injection
- Logging