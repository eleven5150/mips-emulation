#! /bin/bash

update_repos(){
        sudo apt update && sudo apt upgrade -y
}

install_python(){
    sudo apt install software-properties-common -y
    sudo add-apt-repository ppa:deadsnakes/ppa
    sudo apt install python 3.10 -y
}

install_docker(){
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
}

docker_build(){
    sudo docker build -t FPLB .
}

python_env(){
    python3.10 -m venv venv
    source ./venv/bin/activate
    pip install -r requirements.txt
}


main() {
    [ "$UID" -eq 0 ] || exec sudo bash "$0"

    update_repos
    install_python
    install_docker
    docker_build
    python_env

    python . --help
}

main
exit 0