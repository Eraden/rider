#!/bin/env bash

rm -Rf cov
rm -Rf target/debug/*rider*
cargo test --no-run;kcov --exclude-pattern=github.com,target/debug --verify cov $(ls target/debug/rider_editor* | sed "s/\\.d\$//" | head -n1)

