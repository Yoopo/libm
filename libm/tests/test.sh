#! /bin/bash

#echo [+] Run musl test suite
#cd tests/musl-test/libc-test/
#    make --silent;
#    cat math/REPORT
#cd -

echo [+] Run openlibm test suite
cd tests/openlibm-test/openlibm/test
    make
    make bench
    chmod +x test-float test-double bench-openlibm bench-syslibm
    ./test-double
    ./test-float
    echo ===BENCH relibm ===
    ./bench-openlibm
    echo ===BENCH syslibm ===
    ./bench-syslibm
cd -


