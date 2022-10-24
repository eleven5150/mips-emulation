#!/bin/bash

main() {
    sudo apt update && sudo apt upgrade -y

    sudo apt install software-properties-common -y
    sudo add-apt-repository ppa:deadsnakes/ppa
    sudo apt install python 3.10 -y

    sudo apt install docker.io -y
    sudo usermod -a -G docker "$USER"
    echo "$USER ALL=(ALL) NOPASSWD: /usr/bin/dockerd" >> /etc/sudoers
    {
        echo "# Start Docker daemon automatically when logging in if not running.";
        echo "RUNNING=$(pgrep dockerd | grep -v grep)";
        echo "if [ -z ""$RUNNING"" ]; then";
        echo "    sudo dockerd > /dev/null 2>&1 &";
        echo '    disown';
        echo 'fi';
     } >> ~/.bashrc
    docker build -t bench .

    python3.10 -m venv venv
    source ./venv/bin/activate
    pip install -r requirements.txt
    python bencher.py --help
}

main