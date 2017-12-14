# from https://github.com/tsgates/rust.ko/blob/master/kbuild.mk

#############################
# kbuild support build file #
#############################
# This file tells kbuild how to build this project. In order to work it  needs  to  be  included  by
# the kernel build system and executed in the environment exported by the main  `Makefile`  of  this
# project.

# Name of object file to create by Rust
# rust-target := lib${KERNEL_MODULE}.a

# Tell kbuild which files to build
obj-y                 := keys-rust.o
keys-rust-objs        := libkeys.a
# ${KERNEL_MODULE}-objs := $(patsubst %.c,%.o,${C_FILES}) ${rust-target}

# Strip unused symbols from the input object file
EXTRA_LDFLAGS += --gc-sections --entry=init_module --undefined=cleanup_module
EXTRA_LDFLAGS += $(if ${RELEASE},--strip-all)


# Tell kbuild where the source files are
# src := ${BASE_DIR}

# TODO don't HARDCODE
# Actually Xargo not Cargo
CARGO = /home/mdenton/.cargo/bin/xargo


# Fix file paths (since this script will be run from the kbuild's working directory)
# C_FILES    := $(foreach filepath,${C_FILES}   ,${BASE_DIR}/$(filepath))
# RUST_FILES := $(foreach filepath,${RUST_FILES},${BASE_DIR}/$(filepath))
RUST_FILES := $(wildcard src/*.rs)

RCFLAGS = 

# Determine target directory of cargo's module build
CARGO_MOD_DIR := $(src)/target/${UTS_MACHINE}-unknown-none-gnu/$(if ${RELEASE},release,debug)
# Determine target directory of cargo's build script build
CARGO_BLD_DIR := $(src)/target/$(if ${RELEASE},release,debug)

# Build rule for Rust target object
# Note: UTS_MACHINE is the architecture of the target kernel, Rust compilation will  fail  unless  a
#       target file (such as "x86_64-unknown-none-gnu.json") was created for the architecture of the
#       kernel you're trying to compile this module for.
$(obj)/libkeys.a: ${RUST_FILES} FORCE
	cd $(src) && env STD_CLANG_ARGS='${c_flags}' STD_KERNEL_PATH='${CURDIR}' STD_CLANG_FILES='${KERNEL_INCLUDE}' "${CARGO}" rustc $(if ${RELEASE},--release) $(if ${V},--verbose) ${CARGOFLAGS} --target="${UTS_MACHINE}-unknown-none-gnu" -- ${RCFLAGS}
	cp "${CARGO_MOD_DIR}/libkeys.a" $(obj)
