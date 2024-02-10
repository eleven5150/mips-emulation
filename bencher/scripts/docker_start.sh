#! /bin/bash

docker run --rm -d --name bencher -v ./:/app bench:latest sleep 999999999