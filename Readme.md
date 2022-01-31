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

cargo run -- --source ./example --debug
```

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
- [x] Create 404 file
- [ ] Fix authority form starting with `h`
- [ ] Move ThreadConfig to Box or Rc
- [ ] Add config for index file name
- [ ] Catch responses to server with `unknown` response (see RFC7230, 3.1)
- [ ] Add propper semantic handling of headers
- [ ] add a help page!
- [ ] add autocompletion
- [ ] Incrementally fetch header

### Current Defects
- [ ] request with invalid method and path returns 404, should probably return 405 or 501 first
