# Regal

Regal aims to be a simple and container friendly web gallery for picutres.
Completely written in Rust and using frameworks like 
[diesel](http://diesel.rs/), [rocket](https://rocket.rs/) and
[askama](https://github.com/djc/askama), it is heavily focussed on performance.

## Build docker image

To build the docker image, you have to follow two steps. You'll first have to 
build a runnable, self contained binary that does not need any other libraries
installed to run. Thanks to that, the docker image will be built `FROM scratch`
and can be really small (Usually just under 10MB or even less with a simple 
tweak). To build this binary, run the provided script like this:

`$ ./build_linux_musl.sh --release`

This will output the binary into the 
`target/x86_64-unknown-linux-musl/release/regal` file. You'll the build the
docker image by simply using the `Dockerfile`:

`$ docker build .`

You can of course tag the image or add other options to the build process
just as you like. If everything worked fine, you can find your new image
with `$ docker images`.