#!/usr/bin/bash

base=$(pwd)
echo $base
for file in html/pre/*.page
do
	python process.py $file
done
