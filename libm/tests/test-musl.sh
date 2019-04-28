#! /bin/bash

echo [+] Run musl test suite
cd tests/musl-test/libc-test/
    make
    #cat math/REPORT
cd -


