#!/bin/bash
# unagi-package

source "$(dirname "${BASH_SOURCE}")/imosh" || exit 1
eval "${IMOSH_INIT}"

if [ "$#" -ne 1 ]; then
	LOG FATAL "Usage: unagi-package <output zip file>"
fi

unagi_package="$(cd "$(dirname "${BASH_SOURCE}")" && pwd)/unagi-package.php"

mkdir -p "$TMPDIR/unagi_package"
pushd "$TMPDIR/unagi_package" >/dev/null
php "${unagi_package}"
zip '../submissions.zip' *
popd >/dev/null
cp "$TMPDIR/submissions.zip" "$1"
echo -n 'SHA256: '
shasum -a 256 "$TMPDIR/submissions.zip" | cut -f1 -d' '
