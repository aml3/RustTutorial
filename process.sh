#!/usr/bin/bash

base=$(pwd)
echo $base
for file in html/pre/*.page
do
    echo $file
	python process.py $file
done
