
FROM ekidd/rust-musl-builder

ADD . ./
RUN sudo chown -R rust:rust .

CMD ./android-build.sh
CMD cargo build --release 
