
FROM ekidd/rust-musl-builder

ADD . ./
RUN sudo chown -R rust:rust .

CMD cargo build --release