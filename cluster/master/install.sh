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

mkdir -p /home/unagi/.cache
chown unagi:unagi /home/unagi/.cache
chmod 700 /home/unagi/.cache

if ! id ninetan; then
    useradd \
            --home-dir=/home/ninetan \
            --create-home \
            --uid=10002 \
            --user-group \
            --shell=/bin/bash \
            ninetan
fi

chmod 755 /home/ninetan
rm -rf /home/ninetan/.ssh || true
mkdir -p /home/ninetan/.ssh
cp "$(dirname "${BASH_SOURCE}")/../../ssh/unagi.pem" /home/ninetan/.ssh/id_rsa
chmod 600 /home/ninetan/.ssh/id_rsa
ssh-keyscan github.com >> /home/ninetan/.ssh/known_hosts
chown -R ninetan:ninetan /home/ninetan/.ssh

if [ ! -d "/home/ninetan/github" ]; then
    sudo --login --user ninetan \
        git clone git@github.com:imos/icfpc2018.git "/home/ninetan/github"
fi

pushd "/home/ninetan/github"
sudo --user ninetan git pull
popd

mkdir -p /home/ninetan/bin
cp "$(dirname "${BASH_SOURCE}")/poll.sh" /home/ninetan/bin/poll
chmod +x /home/ninetan/bin/poll
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
    ninetan-dropbox
    ninetan-poll
    ninetan-docker
    ninetan-sync
)
for service in "${SERVICES[@]}"; do
    systemctl enable "${service}"
    systemctl start "${service}"
done

###############################################################################
# Set up Dropbox
###############################################################################

if [ ! -d /home/ninetan/Dropbox ]; then
    pushd /home/ninetan
    if [ ! -x .dropbox-dist/dropboxd ]; then
        wget -O - "https://www.dropbox.com/download?plat=lnx.x86_64" | tar xzf -
    fi
    sudo --login --user ninetan .dropbox-dist/dropboxd
    popd
fi

echo '/home/ninetan/Dropbox/ICFPC2018 /home/unagi/dropbox fuse.bindfs nonempty,perms=0777,force-user=ninetan,force-group=ninetan,chown-ignore,chgrp-ignore,chmod-ignore 0 0
' > /etc/fstab.dropbox
cat /etc/fstab.* > /etc/fstab
mount -a
