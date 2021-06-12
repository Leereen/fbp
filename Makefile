all:

build-docker:
	docker build -t fbp:latest -f docker/Dockerfile .

build-database:
	cargo install diesel_cli --no-default-features --features sqlite
	diesel migration run

build-frontend:
	rm -rf frontend/build/
	(cd frontend && npm run build)
	sed -i "s@http://localhost:8010/proxy/@https://freshlybakedprimes.eu/@g" frontend/build/static/js/main.cb3accde.chunk.js

ssl-init:
	./docker/init-letsencrypt.sh

compose-up: build-database
	docker-compose --project-directory . -f docker/compose.yml up
