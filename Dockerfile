
FROM ekidd/rust-musl-builder

ADD . ./
RUN sudo chown -R rust:rust .
RUN ./android-build.sh

CMD cargo build --release 
CMD cargo build --target="aarch64-linux-android" --release
CMD cargo build --target="arm-linux-androideabi" --release 



