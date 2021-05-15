all:

build-docker:
	docker build -t fbp:latest -f docker/Dockerfile .

build-database:
	cargo install diesel_cli --no-default-features --features sqlite
	diesel migration run

ssl-init:
	./docker/init-letsencrypt.sh

compose-up: build-database
	docker-compose --project-directory . -f docker/compose.yml up
