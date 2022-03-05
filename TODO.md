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
#### For refactor
- [ ] move logic to answer http request into new module
- [ ] remove scheduler
- [ ] replace ``std::io`` with tokio
- [ ] implement logic to answer multiple requests

#### For later
- [ ] create readme for dockerhub
- [ ] create default homepage
- [ ] Change default serve directory to something usefull
- [ ] Return 400 Error for invalid request, don't just close connection
- [ ] Add Connection management (rfc7230 chapter 6)
- [ ] limit the number of connections a single client can have to 7 (and make it configurable)
- [ ] add content-type, encoding, location and possibly language (language should prlly be configurable)
- [ ] add (highly optional) ip loggin to stdout as config parameter
- [ ] add a help page!

### Current Defects
- [ ] request with invalid method and path returns 404, should probably return 405 or 501 first

## Hints

### Connection Management 
Issue at hand: our application can easily end up blocked by having to wait for requests (especially with the keep-alive header)

Solution: switching to an event based architecture => using rusts async await

#### Further reading:
- ``https://pages.cs.wisc.edu/~remzi/OSTEP/threads-events.pdf``
- ``https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html``
- ``https://www.zupzup.org/epoll-with-rust/index.html``
