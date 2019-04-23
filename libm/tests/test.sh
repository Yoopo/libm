#! /bin/bash

#echo [+] Run musl test suite
#cd tests/musl-test/libc-test/src/math/
#    make --silent > /dev/null 2> /dev/null;
#    cat REPORT
#cd -

echo [+] Run openlibm test suite
cd tests/openlibm-test/openlibm/test && make
