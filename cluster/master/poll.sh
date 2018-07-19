#!/bin/bash

set -e -u

cd ~/github
git pull
bash "$(dirname "${BASH_SOURCE}")/sync.sh" \
    >/tmp/ninetan-sync-out 2>/tmp/ninetan-sync-err || true
{
  echo 'HTTP/1.0 200 OK'
  echo 'Content-Type: text/html'
  echo
  echo 'OK'
  sleep 10
} | nc -l 18080
