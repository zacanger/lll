#!/bin/sh

git commit --allow-empty -m "$1"
git tag -a "$1" -m "$1"
git push origin master --follow-tags
cargo publish
