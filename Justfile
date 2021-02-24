build-release:
    cargo build --release

build-debug:
    cargo build

test:
    cargo test

bootstrap-python:
    cd connector-agent-python && poetry install
    
setup-python:
    cd connector-agent-python && poetry run maturin develop --release --strip
    
test-python: setup-python
    cd connector-agent-python && poetry run pytest connector_agent_python/tests -v

