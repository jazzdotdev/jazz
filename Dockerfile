
FROM ekidd/rust-musl-builder

ADD . ./
RUN sudo chown -R rust:rust .

ENV PATH="$PWD/NDK/arm/bin:$PATH"
ENV PATH="$PWD/NDK/aarch64/bin:$PATH"

CMD cargo build --release 
CMD cargo build --target="arm-linux-androideabi" --release
CMD cargo build --target="aarch64-linux-android" --release
