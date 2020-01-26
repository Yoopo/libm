mkfile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
mkfile_dir := $(dir $(mkfile_path))

ifndef CARGO_TARGET_DIR
override CARGO_TARGET_DIR = "../"
endif
CRATE_RELEASE_DIR := $(CARGO_TARGET_DIR)release
LDLIBS += -L ${}/home/augustin/.cache/cargo/release -lrelibm -Wl,-rpath=/home/augustin/.cache/cargo/release/

all: shared

shared:
	clang simple.c $(LDLIBS)
static:
	clang simple.c $(CRATE_RELEASE_DIR)/librelibm.a