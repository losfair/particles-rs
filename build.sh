#!/bin/bash

export PRS_BUILD_ID=$(uuidgen)

if [ "$1" = "production" ]
then
    export PRS_BUILD_ID="$PRS_BUILD_ID-production"
    export NODE_ENV="production"
    export WEBPACK_COMMAND="webpack" # builtin minifier does not work

    cargo clean
else
    export PRS_BUILD_ID="$PRS_BUILD_ID-development"
    export NODE_ENV="development"
    export WEBPACK_COMMAND="webpack"
fi

echo "Build ID: $PRS_BUILD_ID"

make || exit 1

node jsbridge/scripts/generate_static_loader.js build/particles.wasm > build/particles-code.js
if [ "$NODE_ENV" = "production" ]
then
    node jsbridge/scripts/minify_all.js
fi
