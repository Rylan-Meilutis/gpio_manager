#!/bin/bash

apt-get update && apt-get install -y cmake device-tree-compiler libfdt-dev
curl -L "https://gitlab.rylanswebsite.com/rylan-meilutis/rppal/-/archive/main/rppal-main.tar.gz" | tar xz
sed -i 's#rppal = { git = "https://gitlab.rylanswebsite.com/rylan-meilutis/rppal.git", branch = "main" }#rppal = { path = "rppal-main" }#' Cargo.toml
tox
TWINE_PASSWORD=${CI_JOB_TOKEN} TWINE_USERNAME=gitlab-ci-token twine upload --repository-url "${CI_API_V4_URL}"/projects/"${CI_PROJECT_ID}"/packages/pypi target/wheels/*
api_token=$PYPI_TOKEN
twine upload -u __token__ -p "${api_token}" target/wheels/*

