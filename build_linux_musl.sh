#!/usr/bin/env bash

Release=''

while (( "$#" )); do
  case "$1" in
  --release)
      Release='YES'
    ;;
  esac
  shift
done

if [ "$Release" == 'YES' ]; then
  docker run -v $PWD:/volume --rm -t clux/muslrust cargo build --release
  cp target/x86_64-unknown-linux-musl/release/regal ./musl_release
else
  docker run -v $PWD:/volume --rm -t clux/muslrust cargo build
  cp target/x86_64-unknown-linux-musl/debug/regal ./musl_debug
fi
