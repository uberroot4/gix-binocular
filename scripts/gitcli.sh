#!/usr/bin/env /bin/bash

echo $1, $(git -C . diff --shortstat $1^ $1)