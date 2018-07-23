#!/bin/bash
# unagi-package

source "$(dirname "${BASH_SOURCE}")/imosh" || exit 1
DEFINE_string nbt_file '' 'NBT file.'
DEFINE_string problem '' 'Problem name.'
DEFINE_string name '' 'Program name.'
eval "${IMOSH_INIT}"

if [ "${FLAGS_nbt_file}" == '' ]; then
	LOG FATAL '--nbt_file=path (e.g., --nbt_file=foo.nbt) flag must be given.'
fi

if [ ! -f "${FLAGS_nbt_file}" ]; then
	LOG FATAL "No such file: ${FLAGS_nbt_file}"
fi

if [ "${FLAGS_problem}" == '' ]; then
	LOG FATAL '--problem=problem_name (e.g., --problem=FA001) flag must be given.'
fi

source_file="$(dirname "${BASH_SOURCE}")/../data/problemsF/${FLAGS_problem}_src.mdl"
target_file="$(dirname "${BASH_SOURCE}")/../data/problemsF/${FLAGS_problem}_tgt.mdl"

if ! [ -f "${source_file}" -o -f "${target_file}" ]; then
	LOG FATAL "No such problem name: ${FLAGS_problem}"
fi

mkdir -p "${TMPDIR}/unagi-submit"
if [ "$(file --mime "${FLAGS_nbt_file}" | grep "charset=binary")" == '' ]; then
	LOG WARNING 'NBT file is not a binary file, so binarizing...'
	"$(dirname "${BASH_SOURCE}")/trace_binarize" \
		"${TMPDIR}/unagi-submit/nbt_file" < "${FLAGS_nbt_file}"
else
	LOG INFO 'NBT file is a binary file.'
	cp "${FLAGS_nbt_file}" "${TMPDIR}/unagi-submit/nbt_file"
fi

sim_args=(
	"$(dirname "${BASH_SOURCE}")/sim"
	-a "${TMPDIR}/unagi-submit/nbt_file")
if [ -f "${source_file}" ]; then sim_args+=(-s "${source_file}"); fi
if [ -f "${target_file}" ]; then sim_args+=(-t "${target_file}"); fi

LOG INFO "Simulator: ${sim_args[*]}"
if "${sim_args[@]}" | grep energy; then
	if [ "${FLAGS_name}" != '' ]; then
		name="${FLAGS_name}"
	else
		name="${USER}-$(basename "${FLAGS_nbt_file}" .nbt)-$(date +'%Y%m%d-%H%M%S')"
	fi
	LOG INFO 'Submitting...'
	nbt_file="${TMPDIR}/unagi-submit/nbt_file" \
	problem_name="${FLAGS_problem}" \
	program_name="${name}" \
		php "$(dirname "${BASH_SOURCE}")/unagi-submit.php"
else
	LOG FATAL "Simulator failed."
fi
