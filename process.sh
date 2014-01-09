#!/usr/bin/bash

base=$(pwd)
echo $base
for file in html/pre/*.page
do
    #Change this if python 2 is your default.
	python2 process.py $file
done
