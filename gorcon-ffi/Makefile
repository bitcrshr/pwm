.PHONY: build

all: build copy

build:
	go build -buildmode=c-shared -o rcon.dylib rcon.go ; \

copy: 
	cp -f rcon.dylib ../web-ssr/src/lib/rcon/dylib/rcon.dylib ; \
	cp -f rcon.h ../web-ssr/src/lib/rcon/dylib/rcon.h ; \