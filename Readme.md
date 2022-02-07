# Throwscape

## Description
Throwscape is a container-native static webserver written in rust.
It attempts to improve upon the currently popular webservers by providing a far simpler and more cloud driven aproach to deploying static content in a container driven context.
In our minds you shouldn't have to mess around creating docker images and configuration files to host your webpages only to find out that you're still carrying around all the added baggage of nginx, apache etc.

## Setup
Currently throwscape is not yet available on dockerhub (See the documentation below on details how to run it).

### Configuration
TBD

## Development
### Getting started
For development purposes, the examples directory includes a couple of webpages that one can use for testing.
In order to get started with the project run:
```bash
git clone https://github.com/grossamos/throwscape.git

cd ./throwscape

cargo run -- --source ./example --debug
```

