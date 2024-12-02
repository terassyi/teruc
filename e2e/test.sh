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

assert_with_output() {
  expected="$1"
  link_target="$2"
  input="$3"
  e2e/teruc "$input" > tmp.s
  cc -c e2e/$link_target.c -o "$link_target".o
  cc "$link_target".o tmp.s -o tmp
  output=$(./tmp)
  if [ "$output" == "$expected" ]; then
    echo "$input => $output"
  else
    echo "$input => $expected" is expected, but got "$output"
    exit 1
  fi
}

assert 0 '0;'
assert 42 '42;'

assert 41 " 12 + 34 - 5 ;"
assert 47 '5+6*7;'
assert 15 '5*(9-6);'
assert 4 '(3+5)/2;'
assert 10 '(-10+20);'
assert 0 '10 - (-10+20);'
assert 1 '1 == 1;'
assert 1 '1 >= 10 - (-10+20);'
assert 2 'a = 1; a = a + 1; return a;'
assert 3 'a = 1; aa = 2; return a + aa;'
assert 2 'a = 1; b = 2; aa = b - a; return a + aa;'
assert 0 'if (1) return 0;'
assert 10 'a = 1; if (a == 1) return 10;'
assert 1 'a = 1; if (a == 0) return 0; else return 1;'
assert 1 'a = 1; if (a == 0) return 0; else if (a == 1) return 1; else return 2;'
assert 10 'a = 0; while (a != 10) a = a + 1; return a;'
assert 10 'b = 0; for(a = 0; a < 10; a = a + 1) b = b + 1; return b;'
assert_with_output "hello from foo" foo "foo();"
assert_with_output "3" add "add(1, 2);"
echo OK
