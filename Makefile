# this file is just to appease Emacs M-x compile

RUSTHOME=$(HOME)/Applications/rust-1.0.0
export PATH=/bin:/usr/bin:$(RUSTHOME)/bin
CARGO=$(RUSTHOME)/bin/cargo

# this is some kind of installation fail, fix later
export DYLD_LIBRARY_PATH=$(RUSTHOME)/lib


build:
	$(CARGO) build

run:
	$(CARGO) run README.md out~
