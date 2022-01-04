# Ultrascape

## Description
Ultrascape is an ultra cool webserver written in rust. 
Its main goal is to create a simple webserver that can be configured and run straigt from the command line. 
No configuration files or complicated setup procedures required.

## Roadmap
### Planned features
1. serving of static webpages based on local files (ex. html, css, etc.) with HTTP/1.1
2. multithreading for better performance (see [Fork-join pattern](https://en.wikipedia.org/wiki/Fork%E2%80%93join_model))
3. caching of recently accessed files (or better yet the most commonly used files)
4. potential support for SSL
5. potential support for HTTP/2.0

### Current ToDo
- [ ] Create basic answer for tcp requests
- [x] Create Configuration framework (additional module)
- [ ] Add concurrency model
- [ ] Add basic HTTP/1.1 support:
    - [ ] Parsing of request 
    - [ ] Translate to sending up file
    - [ ] Add support for binary files like icons and images
    - [ ] Error messages, such as 404, etc.
- [ ] Add basic configuration (port, 404 file, etc.)
