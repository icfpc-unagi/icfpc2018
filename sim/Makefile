UNAME:=$(shell uname)

all: sim
.PHONY: all

../bazel-bin/sim/sim: main.cc
	bazel build -c opt //sim:sim

sim: ../bazel-bin/sim/sim
	cp -f ../bazel-bin/sim/sim ../bin/sim.$(UNAME)
	chmod +x ../bin/sim.$(UNAME)
.PHONY: sim

test: ../bazel-bin/sim/sim
	$< -p ../data/problemsL/LA001_tgt.mdl -a ../data/dfltTracesL/LA001.nbt --logtostderr | grep energy:335123860
	$< -p ../data/problemsL/LA005_tgt.mdl -a ../data/dfltTracesL/LA005.nbt --logtostderr | grep energy:705484076
	$< -p ../data/problemsL/LA010_tgt.mdl -a ../data/dfltTracesL/LA010.nbt --logtostderr | grep energy:257595824
	$< -p ../data/problemsL/LA020_tgt.mdl -a ../data/dfltTracesL/LA020.nbt --logtostderr | grep energy:21353195740
	$< -p ../data/problemsL/LA002_tgt.mdl -a ./tests/tos2.nbt --logtostderr | grep energy:2610292
	$< -p ../data/problemsL/LA050_tgt.mdl -a ./tests/tos50.nbt --logtostderr | grep energy:1340025018
.PHONY: test
