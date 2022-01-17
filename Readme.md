# Ultrascape

## Description
Ultrascape is an ultra cool webserver written in rust. 
Its main goal is to create a simple webserver that can be configured and run straigt from the command line. 
No configuration files or complicated setup procedures required.

## Development
### Getting started
For development purposes, the examples directory includes a couple of webpages that one can use for testing.
In order to get started with the project run:
```bash
git clone https://github.com/grossamos/ultrascape.git

cd ./ultrascape

cargo run -- --source ./example
```

### Dependencies
As of right now, the entire webserver is written in pure (as close to the metal) rust.
Thus no further crates or dependencies outside of the OS exist.



## Roadmap
### Planned features
1. serving of static webpages based on local files (ex. html, css, etc.) with HTTP/1.1
2. multithreading for better performance (see [Fork-join pattern](https://en.wikipedia.org/wiki/Fork%E2%80%93join_model))
3. caching of recently accessed files (or better yet the most commonly used files)
5. support the `Keep alive header`
4. potential support for SSL
5. potential support for HTTP/2.0
6. use of QUIC transmision layer protocol

### Current ToDo
- [x] Create basic answer for tcp requests
- [x] Create Configuration framework (additional module)
- [x] Add concurrency model
- [x] Add basic HTTP/1.1 support:
    - [ ] Incrementally fetching header
    - [ ] Handling of invalid requests:
        - [ ] 404 file 
        - [ ] Have to handle `GET` and `HEAD`, for all others respond with 405 (except when
          unknown, then return 501)
        - [ ] Invalid methods (post etc.)
        - [ ] Non-spec compliant requests
    - [x] Timeouts in case of slow lorris attacks
    - [ ] Propperly send up files (allow for binary, such as images)
    - [ ] Fix authority form for authorities starting with `h`
    - [ ] Catch `..` in request paths somehow
- [ ] Add basic configuration (port, 404 file, etc.)
    - [ ] Change copies of config (threadConfig) to smart pointer (Box)
    - [ ] Add configuration option for index file
- [ ] Add a help page and potentially autocompletion for flags
- [ ] Catch responses to server with `unknown` response (see RFC7230, 3.1)

