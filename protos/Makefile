# Makefile to compile proto files with service/version path preservation,
# and initialize Go modules for each service/version, and generate TypeScript libraries

# Variables
ROOT_DIR := ./src
GO_OUT_DIR := ./compiled/go
TS_OUT_DIR := ./compiled/ts
PROTO_DIRS := $(shell find $(ROOT_DIR) -type d -name 'v*') # Find directories that match service versions
GO_MOD_BASE_NAME := github.com/bitcrshr/pwm/protos/compiled/go

# Ensure the output directory exists
$(shell mkdir -p $(GO_OUT_DIR))
$(shell mkdir -p $(TS_OUT_DIR))

# Protoc Gen Go versions
PROTOC_GEN_GO := $(shell which protoc-gen-go)
PROTOC_GEN_GO_GRPC := $(shell which protoc-gen-go-grpc)
PROTOC_GEN_TS := $(shell which protoc-gen-ts)

# Check if protoc-gen-go and protoc-gen-go-grpc are installed
ifndef PROTOC_GEN_GO
$(error "protoc-gen-go not installed. Please run `go install google.golang.org/protobuf/cmd/protoc-gen-go@latest`")
endif

ifndef PROTOC_GEN_GO_GRPC
$(error "protoc-gen-go-grpc not installed. Please run `go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest`")
endif

ifndef PROTOC_GEN_TS
$(error "protoc-gen-ts not installed. Please run `bun i -g ts-proto`")
endif

# .PHONY to declare non-file targets
.PHONY: all compile_protos clean

# Default target
all: compile_protos

# Compile proto files and initialize Go modules
compile_protos:
	@for dir in $(PROTO_DIRS); do \
		relative_dir=$${dir#$(ROOT_DIR)/}; \
		go_out_dir=$(GO_OUT_DIR)/$${relative_dir}; \
		ts_out_dir=$(TS_OUT_DIR)/$$relative_dir; \
		echo "Compiling proto files in $$dir to $$go_out_dir and $$ts_out_dir..."; \
		mkdir -p $$go_out_dir $$ts_out_dir; \
		protoc --proto_path=$$dir \
			--go_out=$$go_out_dir \
			--go_opt=paths=source_relative \
			--go-grpc_out=$$go_out_dir \
			--go-grpc_opt=paths=source_relative \
			--ts_out=$$ts_out_dir \
			$$dir/*.proto; \
		if [ $$? -ne 0 ]; then \
			echo "Error compiling protos in $$dir" && exit 1; \
		fi; \
		if [ ! -f "$$go_out_dir/go.mod" ]; then \
			echo "Initializing Go module in $$go_out_dir"; \
			(cd $$go_out_dir && go mod init $(GO_MOD_BASE_NAME)/$$relative_dir && go mod tidy) || exit 1; \
		fi; \
	done
	@echo "Compiled all proto files successfully, initialized Go modules, and generated TypeScript libraries!"

# Clean compiled proto files
clean:
	@find $(GO_OUT_DIR) -type d -exec rm -rf {} +;
	@find $(TS_OUT_DIR) -type d -exec rm -rf {} +;
	@echo "Cleaned compiled proto files."
