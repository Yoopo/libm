#! /bin/bash

echo [+] Run musl test suite
cd tests/musl-test/libc-test/
    make --silent > /dev/null 2> /dev/null;
    cat math/REPORT
cd -

echo [+] Run openlibm test suite
cd tests/openlibm-test/openlibm/test && make
