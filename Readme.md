<!-- Header -->
<br />
<p align="center">
  <a href="https://firefly-iii.org/">
    <img src="https://raw.githubusercontent.com/grossamos/throwscape/master/.github/images/Throwscape.svg" alt="Throwscape" width="500" height="100">
  </a>
</p>
<h1></h1>
<p align="center">
  A static webserver designed for containers and written in rust'
  <br />
  <a href="#about-throwscape">About throwscape</a>
  -
  <a href="#getting-started">Getting started</a>
  -
  <a href="#configuration">Configuration</a>
</p>

## About Throwscape
Throwscape is a container-native static webserver written in rust.
It attempts to improve upon the currently popular webservers by providing a simple and cloud driven aproach to deploying static content in a container driven context.

In our minds you shouldn't have to mess around creating docker images and configuration files to host your webpages only to find out that you're still carrying around all the added baggage of nginx, apache etc.

With throwscape you'll have a fully configured webserver in a single command.

## Getting started
The simplest way to run throwscape is with docker.

To get up and running, make sure docker is installed and running on your system (more information on this [here](https://docs.docker.com/engine/install/)).
Then simply pull the image from docker hub and start a container with the following command:
```bash
docker run -d -p 8080:8080 -v your_static_files:/source grossamos/throwscape:latest 
```

And that's it, you should now be able to view your website from [http://localhost:8080](http://localhost:8080) or from the respective IP of your server.

## Configuration
Since you may not like some of our defaults, there are a number of commandline parameters you can pass in to throwscape to make it run how you'd like

#### Source directory
This parameter can be used to set any path (in the container) as a document root.
```bash
--source your file_path
```
Defaults to `/source`

#### Port
```bash
--port 8080
```
Defaults to ``8080``

#### Index file name
Changes the file name used when accessing paths like `<domain>/some_dir/`.
```bash
--index-file-name index.html
```
Defaults to ``index.html``

#### Debug Output
Enables logging of incoming requests and messages on errors that occur
```bash
--debug
```
Is not applied by default

## Development
### Creating a development environment
First, ensure that rustc, cargo and any other nessicary components of the rust toolchain are installed.
To make testing configurations more reproducable, the application is usually tested using the files found in the ``examples`` directory.

In order to compile and run the project with the provided test-website run:
```bash
git clone https://github.com/grossamos/throwscape.git

cd ./throwscape

cargo run -- --source ./example --debug
```

### Building from scatch
In order to build throwscape (ex. for use in a different container technology) run the following command:

```bash
cargo build --release --target x86_64-unknown-linux-musl
```

If you recive an error about musl missing, you can install it using rustup:
```bash
rustup target add x86_64-unknown-linux-musl
```

The added flags ensure that the binary is statically compiled (including the c-library, which is usally dynamically linked in rust).

#### Docker
To build the Docker image from scratch make sure that you've compiled the project, then run:
```bash
docker build --no-cache -t grossamos/throwscape:latest .
```
