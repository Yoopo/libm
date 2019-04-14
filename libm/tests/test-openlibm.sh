#! /bin/bash

set -o errexit
set -o pipefail
set -o nounset
# set -o xtrace

SCRIPT_DIR="$( cd "$(dirname "$0")" ; pwd -P )"
REPO_DIR="${SCRIPT_DIR}/openlibm/"
TEST_DIR="${REPO_DIR}/test"
CRATE_RELEASE_DIR="${CARGO_TARGET_DIR:-${SCRIPT_DIR}/..}/release/"

# clone and adapt test to be build against relibm
if [ ! -d "${REPO_DIR}" ]; then
  cd ${SCRIPT_DIR}
    git clone https://github.com/JuliaMath/openlibm.git
    cd ${TEST_DIR}
        sed -i 's#OPENLIBM_LIB = -L.. -lopenlibm#OPENLIBM_LIB = -lpthread -ldl -L ${SCRIPT_DIR} -lrelibm -Wl,-rpath=/home/augustin/.cache/cargo/release/#' Makefile
    cd -
  cd -
fi

# make sure we have a library to test
cargo build --release
cd ${TEST_DIR}
    make
    make bench
    chmod +x test-float test-double bench-openlibm bench-syslibm
    echo === Running tests ===
    ./test-double || true
    ./test-float  || true
    echo === BENCH relibm ===
    ./bench-openlibm
    echo === BENCH syslibm ===
    ./bench-syslibm
cd -
