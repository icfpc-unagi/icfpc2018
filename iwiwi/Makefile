UNAME:=$(shell uname)

all: trace_binarize
.PHONY: all

trace_binarize: trace_binarize.cpp
	c++ -std=c++14 -O3 -o ../bin/trace_binarize.$(UNAME) trace_binarize.cpp
.PHONY: trace_binarize
