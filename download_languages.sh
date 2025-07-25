#!/bin/sh
# This script downloads the languages.yml file from the GitHub Linguist repository.
# It should be run from the root of the crate.
set -e
echo "Downloading languages.yml..."
curl -L "https://raw.githubusercontent.com/github-linguist/linguist/main/lib/linguist/languages.yml" -o "languages.yml"
echo "Download complete."
