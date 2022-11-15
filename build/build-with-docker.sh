#!/bin/bash

source build/set-build-description.sh && \
docker build --progress=plain --build-arg VECTOR_BUILD_DESC="$VECTOR_BUILD_DESC" -t "$(cargo pkgid vector | awk '{split($0,v,"#");print v[2]}')"_"$(git rev-parse --short HEAD)"_"$(date +%Y%m%d)" -f build/builder.Dockerfile .
docker image prune -f
