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
	$< ../data/dfltTracesL/LA001.nbt -t ../data/problemsL/LA001_tgt.mdl --logtostderr | grep energy:335123860
	$< ../data/dfltTracesL/LA005.nbt -t ../data/problemsL/LA005_tgt.mdl --logtostderr | grep energy:705484076
	$< ../data/dfltTracesL/LA010.nbt -t ../data/problemsL/LA010_tgt.mdl --logtostderr | grep energy:257595824
	$< ../data/dfltTracesL/LA020.nbt -t ../data/problemsL/LA020_tgt.mdl --logtostderr | grep energy:21353195740
	$< ./tests/tos2.nbt -t ../data/problemsL/LA002_tgt.mdl --logtostderr | grep energy:2610292
	$< ./tests/tos50.nbt -t ../data/problemsL/LA050_tgt.mdl --logtostderr | grep energy:1340025018
	$< ./tests/void.nbt -r 10
	$< ./tests/gvoid.nbt -s ../data/problemsF/FD001_src.mdl --logtostderr | grep energy:9475625
.PHONY: test
