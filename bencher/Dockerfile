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
RUN wget https://github.com/JetBrains/kotlin/releases/download/v1.7.10/kotlin-compiler-1.7.10.zip \
    && apt install unzip \
    && unzip kotlin-compiler-1.7.10.zip -d /root \
    && rm -f kotlin-compiler-1.7.10.zip \
    && ln -s /root/kotlinc/bin/kotlinc /usr/local/bin/kotlinc   
# C Sharp
RUN apt install -y aspnetcore-runtime-6.0 \
    && apt install -y dotnet-sdk-6.0

