#!/usr/bin/env bash
cat <<EOF
#################################################################
Welcome.  You need to run one of the scripts in /app/bin:
$(ls /app/bin | sed 's/^/ * /')
#################################################################
EOF
