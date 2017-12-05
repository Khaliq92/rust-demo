#!/bin/bash
set -x

# clone my utah test branch, download and extract test data

{
    git clone -b tsv git@github.com:stevekm/utah.git ../utah
} && {
    wget http://hgdownload.cse.ucsc.edu/goldenPath/hg38/database/ncbiRefSeq.txt.gz
} && {
    gunzip ncbiRefSeq.txt.gz
}