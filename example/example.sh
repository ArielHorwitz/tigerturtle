#! /bin/bash
set -e

config_file=example.toml
config_keys=(foo _bar numbers__pi)
config_default='
foo = "eggs"
bar = "spam"

[numbers]
pi = 3.1415926535
'
tt_args=(--output-prefix "config__" $config_file -- ${config_keys[@]})
tt_out=$(mktemp 'tt_out.XXXXXXXXXX'); tt_err=$(mktemp 'tt_err.XXXXXXXXXX')
if tigerturtle -D "$config_default" ${tt_args[@]} >$tt_out 2>$tt_err; then
    # For debugging: echo "$(<$tt_out)" >&2
    eval $(<$tt_out); rm $tt_out; rm $tt_err;
else
    echo "$(<$tt_err)" >&2; rm $tt_out; rm $tt_err; exit 1;
fi

echo foo: $config__foo
echo bar: $config__bar
echo pi: $config__numbers__pi

