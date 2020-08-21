#!/bin/bash
docker-compose exec api cargo fmt
docker-compose exec web yarn run prettier-format --loglevel warn
