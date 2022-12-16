.PHONY: build

build:
	rm -rf ./build
	mkdir ./build
	go build -o build/main main.go

run:
	nodemon -e go -x 'go run main.go || exit 1' --signal SIGTERM
