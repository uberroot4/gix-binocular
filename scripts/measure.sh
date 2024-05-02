#!/usr/bin/env /bin/bash

# Tests are run using [hyperfine](https://github.com/sharkdp/hyperfine).
# If not explicitly mentioned, they are run on RAMDisk.

hyperfine --warmup 2 --runs 10 --export-json gitinfo.binocular.1.json --export-csv gitinfo.binocular.1.csv --parameter-list repo Binocular --parameter-list workers 1,2,3,4,5,6,7,8 --parameter-list algorithm histogram,myers,myers-minimal './target/release/gitinfo --git-dir {repo} --threads {workers} --algorithm {algorithm}'
# hyperfine --warmup 2 --runs 10 --export-json gitinfo.1.json --parameter-list repo Binocular --parameter-list workers 1,2,3,4 './gitinfo --git-dir {repo} --threads {workers}'