#!/bin/bash

apt-get update && apt-get install -y cmake device-tree-compiler libfdt-dev
tox
TWINE_PASSWORD=${CI_JOB_TOKEN} TWINE_USERNAME=gitlab-ci-token twine upload --repository-url "${CI_API_V4_URL}"/projects/"${CI_PROJECT_ID}"/packages/pypi target/wheels/*
api_token=$PYPI_TOKEN
twine upload -u __token__ -p "${api_token}" target/wheels/*

