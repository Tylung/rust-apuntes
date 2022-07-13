#!/bin/bash

echo "Run $1!"

sleep 1

clear

rustc $1 -o script

./script

rm ./script
