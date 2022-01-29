# api_request

Usage: `cargo run [LOAD_OPTION] [VERBOSE_OPTION] --release`

`LOAD_OPTION`:
  `load [FILENAME]`\
`VERBOSE_OPTION`:
  `verbose`

Example:\
`cargo run load ids.txt verbose --release` to request information for IDs in ids.txt.\
`cargo run verbose --release` to request default IDs from 0 to 200.

Results are stored in `infos.txt` and each new run overwrites the file from the beginning.
