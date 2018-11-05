export RUNNING_HOME=$(pwd)
sudo apt-get update && \
    sudo apt-get install -yq sudo curl file build-essential wget git g++ cmake pkg-config bison flex \
                        unzip lib32stdc++6 lib32z1 python autotools-dev automake autoconf libtool \
                        gperf xsltproc docbook-xsl

# # Rust & Cargo
curl https://sh.rustup.rs -sSf | sh -s -- -y
export PATH="/root/.cargo/bin:$PATH"
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android

# Android NDK and toolchain
cd /usr/local && \
        wget  https://dl.google.com/android/repository/android-ndk-r16b-linux-x86_64.zip && \
        unzip -q android-ndk-r16b-linux-x86_64.zip && \
        rm android-ndk-r16b-linux-x86_64.zip
export NDK_HOME="/usr/local/android-ndk-r16b"

# build toolchain for arm 
${NDK_HOME}/build/tools/make-standalone-toolchain.sh \
--arch=arm --install-dir=/opt/NDK/arm --stl=libc++ --platform=android-26

# build toolchain for aarch64 
${NDK_HOME}/build/tools/make-standalone-toolchain.sh \
--arch=arm64 --install-dir=/opt/NDK/aarch64 --stl=libc++ --platform=android-26

echo '[target.arm-linux-androideabi] 
ar = "arm-linux-androideabi-ar" 
linker = "arm-linux-androideabi-clang" 
 
[target.aarch64-linux-android] 
ar = "aarch64-linux-android-ar" 
linker = "aarch64-linux-android-clang"' > ~/.cargo/config
~
~

export PATH="$PATH:/opt/NDK/arm/bin" 
export PATH="$PATH:/opt/NDK/aarch64/bin"
cd $RUNNING_HOME 
