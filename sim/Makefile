UNAME:=$(shell uname)

all: sim
.PHONY: all

../bazel-bin/sim/sim: main.cc
	bazel build -c opt //sim:sim

sim: ../bazel-bin/sim/sim
	cp -f ../bazel-bin/sim/sim ../bin/sim.$(UNAME)
	chmod +x ../bin/sim.$(UNAME)
.PHONY: sim
