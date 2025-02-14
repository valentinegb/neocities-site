#!/bin/bash

file=`echo dist/.stage/neocities_site-*_bg.wasm`

mv "$file" ${file/.wasm/.txt}

sed -e 's/_bg.wasm/_bg.txt/g' dist/.stage/index.html > tmp
mv tmp dist/.stage/index.html
