#! /bin/bash

main() {
    [ "$UID" -eq 0 ] || exec sudo bash "$0"

    docker stop $(docker ps -a -q)
    docker rm $(docker ps -a -q)
}

main
exit 0