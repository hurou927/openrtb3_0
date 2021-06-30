#!/bin/bash

if [ $# != 2 ]; then
  echo $0 [fileName] [structName]
  exit 1;
fi

fileName=$1
structName=$2

filePath=./src/adcom/$fileName.rs

content="
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ${structName} {}
"

touch $filePath
echo "$content" > $filePath
echo "pub mod $fileName;" >> ./src/adcom.rs

