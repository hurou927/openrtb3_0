#!/bin/bash

if [ $# != 3 ]; then
  echo $0 [fileName] [liestName] [elem=num]
  exit 1;
fi

fileName=$1
listName=$2
sample=$3


filePath=./src/adcom/$fileName.rs

content="
use crate::rtb_type;

rtb_type! {
$listName,
500,
$sample
}
"

touch $filePath
echo "$content" > $filePath
echo "pub mod $fileName;" >> ./src/adcom.rs

