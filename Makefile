docker-build:
	docker build -t wbtools:dev --build-arg USER=${USER} --build-arg UID=`id -u` - < Dockerfile
compile:
	cargo build
