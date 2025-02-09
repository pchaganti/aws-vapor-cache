#!/bin/sh
mkdir -p package
python3 -m pip install -r requirements.txt -t package
cp lambda_function.py package
(cd package && zip -r ../package.zip .)
rm -rf package
 