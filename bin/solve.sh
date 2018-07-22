#!/bin/bash
# Runs solver.

source "$(dirname "${BASH_SOURCE}")/imosh" || exit 1
DEFINE_enum --values=chokudai,wata solver chokudai 'Solver to choose.'
DEFINE_string version '' 'Version.'
DEFINE_bool binary false 'Output as binary.'
DEFINE_string --alias=o output '' 'Output file.'
DEFINE_string problem 'FA001' 'Problem to solve.'
DEFINE_bool skip_postprocess false 'Skip post process.'
DEFINE_bool simulate false 'Simulate.'
eval "${IMOSH_INIT}"

problem_file="$(dirname "${BASH_SOURCE}")/../data/problemsF/${FLAGS_problem}"

args=("${problem_file}_tgt.mdl" "$@")

run_solver() {
	if [ "${FLAGS_solver}" == 'chokudai' ]; then
		version="${FLAGS_version}"
		if [ "${FLAGS_version}" == '' ]; then
			version="$(cd "$(dirname "${BASH_SOURCE}")/chokudai-solver" &&
			           ls | sort -r | head -n 1)"
			version="${version//.exe/}"
			LOG INFO "Automatically detected version ${version}."
		else
			LOG INFO "Running version ${version}."
		fi
		mono "$(dirname "${BASH_SOURCE}")/chokudai-solver/${version}.exe" "${args[@]}"
		return
	fi
	LOG FATAL "Unknown solver: ${FLAGS_solver}"
}

run_with_postprocess() {
	if (( FLAGS_skip_postprocess )); then
		run_solver
	else
		run_solver | "$(dirname "${BASH_SOURCE}")/run_postproc" "${problem_file}_tgt.mdl" /dev/stdin
	fi
}

run_with_binarizer() {
	if (( FLAGS_binary )); then
		run_with_postprocess | "$(dirname "${BASH_SOURCE}")/trace_binarize" /dev/stdout
	else
		run_with_postprocess
	fi
}

run_with_simulator() {
	if (( FLAGS_simulate )); then
		FLAGS_binary=1
		run_with_binarizer | \
			"$(dirname "${BASH_SOURCE}")/sim" \
				--alsologtostderr="${FLAGS_alsologtostderr}" \
				--logtostderr="${FLAGS_logtostderr}" \
				-a /dev/stdin -p "${problem_file}_tgt.mdl"
	else
		run_with_binarizer
	fi
}

run() {
	if [ "${FLAGS_output}" == '' ]; then
		run_with_simulator
	else
		run_with_simulator > "${FLAGS_output}"
	fi
}

RUST_BACKTRACE=1 run
