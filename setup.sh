#!/bin/sh

cargo build --release

if pgrep "sha-on-crack" > /dev/null; then
  sha-on-crack "$@"
else
  echo "Error: sha-on-crack binary is not running"
fi

chmod +x sha_on_crack
