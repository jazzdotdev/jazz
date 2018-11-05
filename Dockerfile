
FROM ekidd/rust-musl-builder

ADD . ./
RUN sudo chown -R rust:rust .
ENV NDK_HOME="android-ndk-r16b"
ENV PATH="~/NDK/arm/bin:${PATH}"
ENV PATH="~/NDK/aarch64/bin:${PATH}"

CMD cargo build --release 
CMD cargo build --target="aarch64-linux-android" --release
CMD cargo build --target="arm-linux-androideabi" --release 



