#!/usr/bin/bash

python3 -m venv venv --system-site-packages && source venv/bin/activate
uv pip install tox
tox
TWINE_PASSWORD=${CI_JOB_TOKEN} TWINE_USERNAME=gitlab-ci-token python -m twine upload --repository-url "${CI_API_V4_URL}"/projects/"${CI_PROJECT_ID}"/packages/pypi target/wheels/*
api_token=$PYPI_TOKEN
twine upload -u __token__ -p "${api_token}" target/wheels/*

