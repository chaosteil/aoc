#!/bin/bash
set -e

prefix=aoc

if [[ -z "$1" ]]; then
  echo "creates a new copy from template/ into a new directory named ${prefix}[suffix]/"
  echo
  echo "usage:"
  echo "$ $0 [suffix]"
  echo
  echo "for example '$0 1' creates a new directory named ${prefix}1/ with the contents of template/"
  exit 1
fi
num=$1
cp -rf template/ "$prefix$num/"
