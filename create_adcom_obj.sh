#!/bin/bash

if [ $# != 2 ]; then
  echo $0 [fileName] [structName]
  exit 1;
fi

fileName=$1
structName=$2


filePath=./src/adcom/$fileName.rs
extName=${structName}Ext

content="
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ${structName} {
  ext: Option<${extName}>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ${extName} {}
"

touch $filePath
echo "$content" > $filePath
echo "pub mod $fileName;" >> ./src/adcom.rs

