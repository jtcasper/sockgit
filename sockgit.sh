#!/bin/bash
if [ $# -lt 2 ]
then
  echo $#
  echo "Usage: sockgit <host> <repo-name>"
  exit 1
fi
if [ -z $PORT ]
then
  PORT=9119
fi
echo $2 | timeout 1 netcat $1 $PORT
