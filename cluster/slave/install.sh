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
for program in ninetan-sync ninetan-score-daemon ninetan-execute-daemon; do
    cp "$(dirname "${BASH_SOURCE}")/${program}.sh" "/home/ninetan/bin/${program}"
    chmod +x "/home/ninetan/bin/${program}"
done

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
    ninetan-execute-daemon\@{1..16}
    ninetan-score-daemon\@{1..4}
)
systemctl enable "${SERVICES[@]}"
systemctl start "${SERVICES[@]}"
