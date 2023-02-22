format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

build-container:
	//build the container
	docker build -t mockingjayactix .

run-container:
	docker run -it --rm -p 8080:8080 mockingjayactix

release:
	cargo build --release

all: format lint test run
