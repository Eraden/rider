#!/bin/env bash

nvm use 11.0.0
here=$(pwd)
target="cov/$(ls ${here}/cov | grep editor | grep "\\." | head -n1)"
if [[ -e "${target}" ]];
then
    cd ${target} && echo "here: $(pwd)" && serve . && cd ${here}
else
    echo "Target: '${target}' does not exists!"
fi

