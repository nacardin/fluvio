#!/usr/bin/env bash
set -e

if [ -n "$MINIKUBE_DOCKER_ENV" ]; then
  eval $(minikube -p minikube docker-env)
fi

tmp_dir=$(mktemp -d -t fluvio-docker-image-XXXXXX)
mkdir $tmp_dir/fluvio-src
cp Cargo.toml $tmp_dir/fluvio-src
cp Cargo.lock $tmp_dir/fluvio-src
cp VERSION $tmp_dir/fluvio-src
cp -R src $tmp_dir/fluvio-src
cp -R examples $tmp_dir/fluvio-src
cp -R tests $tmp_dir/fluvio-src
cp $(dirname $0)/fluvio.Dockerfile $tmp_dir/Dockerfile
cd $tmp_dir
ls -l $tmp_dir
docker build --build-arg RELEASE_FLAG -t infinyon/fluvio:$DOCKER_TAG .
rm -rf $tmp_dir
