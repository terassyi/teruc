#!/bin/bash

assert() {
  expected="$1"
  input="$2"

  e2e/teruc "$input" > tmp.s
  cc -o tmp tmp.s
  ./tmp
  actual="$?"

  echo "------------------------------"
  cat tmp.s
  echo "------------------------------"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 0
assert 42 42

assert 41 " 12 + 34 - 5 "
assert 47 '5+6*7'
assert 15 '5*(9-6)'
assert 4 '(3+5)/2'
assert 10 '(-10+20)'
assert 0 '10 - (-10+20)'
assert 1 '1 == 1'
assert 1 '1 >= 10 - (-10+20)'
assert 3 'a = 1; aa = 2; return a + aa'
assert 2 'a = 1; b = 2; aa = b - a; return a + aa'
assert 0 'if (1) return 0'
assert 10 'a = 1; if (a == 1) return 10'
echo OK
