#!/bin/bash

assert() {
  expected="$1"
  input="$2"

  e2e/teruc "$input" > tmp.s
  cc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 0
assert 4 4

assert 4 " 1 + 8 - 5 "

echo OK
