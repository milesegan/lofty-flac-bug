# Simple repro for possible lofty-rs flac bug

The sample file seems to be corrupted after adding a genre tag.
Adding a genre tag via the metaflac commandline tool does not corrupt the file.

To reproduce run the shell command `cargo run`.
