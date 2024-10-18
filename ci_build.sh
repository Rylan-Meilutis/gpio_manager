python3 -m venv venv && source venv/bin/activate

maturin build --release --python python3.8 --python python3.9 --python python3.10 --python python3.11 --python python3.12

TWINE_PASSWORD=${CI_JOB_TOKEN} TWINE_USERNAME=gitlab-ci-token python -m twine upload --repository-url "${CI_API_V4_URL}"/projects/"${CI_PROJECT_ID}"/packages/pypi target/wheels/*
api_token=$PYPI_TOKEN
twine upload -u __token__ -p "${api_token}" target/wheels/*

