#!/usr/bin/env bash

EXPECTED="= 0xe4d909c290d0fb1ca068ffaddf22cbd0"
ACTUAL=`LD_LIBRARY_PATH=./target/debug ./bin/md5`
printf "The two strings below should match:\n  expected $EXPECTED\n  actual   $ACTUAL\n"

if [ "$ACTUAL" != "$EXPECTED" ]; then
  printf "No match!\n"
  exit 1
fi

printf "Match!\n"
exit 0
