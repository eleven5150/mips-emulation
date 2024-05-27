docker run --name mips_build_run --rm -dit -v ./:/app mips_build sleep 999999999
docker exec -it mips_build_run /bin/bash
apt update
apt install make
cd /app