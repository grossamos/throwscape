# TODO

## Description
Aside from issues, this file contains a rough road map for future development of throwscape.To be determined

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
- [x] Add config for index file name
- [x] Move ThreadConfig to Box or Rc
- [x] Fix authority form starting with `h`
- [x] Catch responses to server with `unknown` response (see RFC7230, 3.1)
- [x] Incrementally fetch header (header is not just ignored at this point)
- [ ] Add propper semantic handling of headers
- [ ] Return 400 Error for invalid request, don't just close connection
- [ ] add autocompletion
- [ ] add a help page!

### Current Defects
- [ ] request with invalid method and path returns 404, should probably return 405 or 501 first
