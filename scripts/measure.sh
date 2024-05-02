#!/usr/bin/env /bin/bash

# Tests are run using [hyperfine](https://github.com/sharkdp/hyperfine).
# If not explicitly mentioned, they are run on RAMDisk.

hyperfine --warmup 1 --runs 5 --export-json gitinfo.binocular.1.json --export-csv gitinfo.binocular.1.csv --parameter-list repo Binocular,dubbo --parameter-list workers 1,2,3,4,5,6,7,8 --parameter-list algorithm histogram,myers,myers-minimal './target/release/gitinfo --git-dir {repo} --threads {workers} --algorithm {algorithm}'