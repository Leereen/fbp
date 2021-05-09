#/usr/bin/env bash

set -e

for route in '/' '/at_position/12' '/first_greater_than/31' '/greater_than/31';
do
    curl -q http://localhost:8000/$route
    echo
done
