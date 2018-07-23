#!/bin/bash
# unagi-package

source "$(dirname "${BASH_SOURCE}")/imosh" || exit 1
eval "${IMOSH_INIT}"

target="unagi-$(date +'%Y%m%d-%H%M%S').zip"
unagi_package="$(cd "$(dirname "${BASH_SOURCE}")" && pwd)/unagi-package.php"

mkdir -p "$TMPDIR/unagi_package"
pushd "$TMPDIR/unagi_package" >/dev/null
php "${unagi_package}"
zip -e -P 6a0b30e3c9c24af2b7bf098ecc58be99 '../submissions.zip' *
popd >/dev/null
cp "$TMPDIR/submissions.zip" "${target}"
echo -n 'SHA256: '
shasum -a 256 "$TMPDIR/submissions.zip" | cut -f1 -d' '
echo "Outputted to ${target}."
