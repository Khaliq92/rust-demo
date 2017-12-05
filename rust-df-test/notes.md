---
title: "Rust DF Demo"
author: "Stephen Kelly"
date: "12/5/2017"
output: html_document
---
Test implementation of df processing in Rust for speed improvements compared to R

this is a good use case because in this db, the genreation of md5sums takes a long time

maybe Rust can make it go faster

or maybe not? Does the R digest lib use C md5 algo?? not sure but try it anyway

Rust data frame lib:
https://github.com/kernelmachine/utah

https://docs.rs/utah/0.1.2/utah/

- (externally) get .csv of all variants from all runs

- load into Rust Utah df

- figure out how to stringify all df entries with timestamp & md5sum them all, similar to R workflow

- track time to completion of these steps

# Rust Notes

```
cargo new --bin rust-df-test

cargo run
```

# Dataset

http://hgdownload.cse.ucsc.edu/goldenPath/hg38/database/

```
wget http://hgdownload.cse.ucsc.edu/goldenPath/hg38/database/ncbiRefSeq.txt.gz
gunzip ncbiRefSeq.txt.gz
```



```
$ cat Cargo.toml
[package]
name = "rust-df-test"
version = "0.1.0"
authors = ["Stephen Kelly <stevekm@users.noreply.github.com>"]

[dependencies]
utah = { path = "utah"}


$ cat old_Cargo.toml
[package]
name = "rust-df-test"
version = "0.1.0"
authors = ["Stephen Kelly <stevekm@users.noreply.github.com>"]

[dependencies]
utah="0.1.2"
csv = "1.0.0-beta.5"
```

# Other resources

http://blog.burntsushi.net/csv/

https://docs.rs/csv/1.0.0-beta.5/csv/
