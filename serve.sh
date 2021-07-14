#!/bin/bash
cd "$( dirname "${BASH_SOURCE[0]}" )" || exit 1
echo "Serving site from $PWD"
rbenv exec bundle exec jekyll serve
