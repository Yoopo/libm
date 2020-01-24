mkfile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
mkfile_dir := $(dir $(mkfile_path))

ifndef CARGO_TARGET_DIR
override CARGO_TARGET_DIR = "../"
endif
CRATE_RELEASE_DIR := $(CARGO_TARGET_DIR)release
all:
	$(CC) simple.c -lpthread -ldl $(CRATE_RELEASE_DIR)/librelibm.a