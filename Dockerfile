
FROM ekidd/rust-musl-builder

COPY . ./
RUN sudo chown -R rust:rust .

CMD cargo build --release 
