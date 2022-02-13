FROM scratch

MAINTAINER Amos Gross

COPY target/x86_64-unknown-linux-musl/release/throwscape /

EXPOSE 8080

ENTRYPOINT ["/throwscape"]

CMD ["--source", "/source"]

