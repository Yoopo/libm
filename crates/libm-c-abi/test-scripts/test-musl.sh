#! /bin/bash

set -o errexit
set -o pipefail
set -o nounset
# set -o xtrace

SCRIPT_DIR="$( cd "$(dirname "$0")" ; pwd -P )"
REPO_DIR="${SCRIPT_DIR}/libc-test/"
CRATE_RELEASE_DIR="${CARGO_TARGET_DIR:-${SCRIPT_DIR}/..}/release/"

if [ ! -d "${REPO_DIR}" ]; then
  cd ${SCRIPT_DIR}
    git clone git://nsz.repo.hu:45100/repo/libc-test
    cd ${REPO_DIR}
        
        echo "LDLIBS += -L ${CRATE_RELEASE_DIR}/release -lrelibm -Wl,-rpath=${CRATE_RELEASE_DIR}" | \
            cat config.mak.def - > config.mak
    cd -
  cd -
fi

# make sure we have a library test
cargo build --release
echo [+] Run musl test suite
cd ${REPO_DIR}
    make
    #cat math/REPORT
cd -