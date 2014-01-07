#!/usr/bin/bash

base=$(pwd)
echo $base
for file in html/*.html
do
	echo "Processing $file"
	python process.py $file
done
