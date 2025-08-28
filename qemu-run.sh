#!/usr/bin/env bash

qemu-system-arm \
  -M ast1030-evb \
  -kernel target/ast1060-mctp-echo/dist/default/final.bin \
  -nographic
