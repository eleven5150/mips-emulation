FROM ubuntu:22.04

RUN apt update && apt -y upgrade \
    && DEBIAN_FRONTEND=noninteractive TZ=Etc/UTC apt -y install tzdata
# for new repo
RUN apt -y install software-properties-common
# for JS
RUN apt install -y nodejs npm \
    && npm install -g ts-node
#python cpython and pypy
RUN apt install -y python3 pypy3
# OpenJDK for Kotlin
RUN apt install -y openjdk-11-jdk
# Rust
RUN apt install -y rustc
# C
RUN apt install -y gcc
RUN apt install -y wget
# Kotlin JVM
RUN wget https://github.com/JetBrains/kotlin/releases/download/v1.7.21/kotlin-compiler-1.7.21.zip \
    && apt install unzip \
    && unzip kotlin-compiler-1.7.21.zip -d /root \
    && rm -f kotlin-compiler-1.7.21.zip \
    && ln -s /root/kotlinc/bin/kotlinc /usr/local/bin/kotlinc
# Kotlin Native
RUN wget https://github.com/JetBrains/kotlin/releases/download/v1.7.21/kotlin-native-linux-x86_64-1.7.21.tar.gz \
    && apt install tar \
    && tar -xzf kotlin-native-linux-x86_64-1.7.21.tar.gz -C /root \
    && rm -f kotlin-native-linux-x86_64-1.7.21.tar.gz \
    && ln -s /root/kotlin-native-linux-x86_64-1.7.21/bin/kotlinc-native /usr/local/bin/kotlinc-native \
    && ln -s /root/kotlin-native-linux-x86_64-1.7.21/bin/run_konan /usr/local/bin/run_konan \
    && mkdir /tmp/test_build \
    && echo 'fun main() { \
        println("Hello World") \
    }' > /tmp/test_build/test.kt \
    && kotlinc-native /tmp/test_build/test.kt -o /tmp/test_build/test.elf \
    && rm -rf /tmp/test_build
# C Sharp
RUN apt install -y aspnetcore-runtime-6.0 \
    && apt install -y dotnet-sdk-6.0
# Cython
RUN apt install -y cython3

