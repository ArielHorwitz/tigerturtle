tigerturtle lets you safely parse and evaluate toml files as bash variables.

## Why?
I use bash scripts a lot. I like to load configurations. I dislike the idea of simply running `eval $(cat some_file)`. This allows me to parse a TOML file and safely evaluate just the keys I want.

## How?
Install using `cargo install tigerturtle`.

Run `tigerturtle --generate >> some_script.sh` and modify. If you're unsure how to proceed from here, I recommend running `cat $tt_out_file` before the `eval` command.

## I can break this
Feel free to report an issue.
