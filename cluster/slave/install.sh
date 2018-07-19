#!/bin/bash

set -e -u -x

apt update
apt install -y docker.io bindfs nfs-common

if ! id unagi; then
    useradd \
            --home-dir=/home/unagi \
            --create-home \
            --uid=10001 \
            --user-group \
            --shell=/bin/bash \
            unagi
fi
echo 'unagi ALL=(ALL:ALL) NOPASSWD: ALL' > /etc/sudoers.d/unagi

if ! id ninetan; then
    useradd \
            --home-dir=/home/ninetan \
            --create-home \
            --uid=10002 \
            --user-group \
            --shell=/bin/bash \
            ninetan
fi

mkdir -p /home/ninetan/bin
cp "$(dirname "${BASH_SOURCE}")/ninetan-sync.sh" /home/ninetan/bin/ninetan-sync
chmod +x /home/ninetan/bin/ninetan-sync

###############################################################################
# Set up NFS
###############################################################################

mkdir -p /efs
if [ ! -f /etc/fstab.orig ]; then
    cp /etc/fstab /etc/fstab.orig
fi
echo 'fs-32b65013.efs.ap-northeast-1.amazonaws.com:/ /efs nfs4 nfsvers=4.1,rsize=1048576,wsize=1048576,hard,timeo=600,retrans=2,noresvport 0 0' > /etc/fstab.efs
cat /etc/fstab.* > /etc/fstab
mount -a

###############################################################################
# Set up Docker
###############################################################################

if [ "$(sudo --login docker info | grep unagi2018)" == '' ]; then
    sudo --login docker login --username unagi2018
fi

###############################################################################
# Set up daemon services
###############################################################################

rm /etc/systemd/system/ninetan-* || true
cp "$(dirname "${BASH_SOURCE}")"/systemd/ninetan-* /etc/systemd/system/
systemctl daemon-reload

SERVICES=(
    ninetan-docker
    ninetan-sync
)
for service in "${SERVICES[@]}"; do
    systemctl enable "${service}"
    systemctl start "${service}"
done
