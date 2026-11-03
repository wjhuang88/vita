#!/bin/sh
set -eu
ROOT="${1:-.}"
echo "profile recommendation: high-risk"
echo "signals: bootable media; physical disk writes; privileged OS actions; external model credentials; Agent-triggered system mutations"
echo "override note: code volume is not yet meaningful because this is a new repository; risk signals determine governance depth."
