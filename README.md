# USAGE

Run the docker container as in `commands.sh`.
Enter it.
Start the TypeDB server: `typedb server`
  (maybe after visiting `/opt/typedb/core/`).

Enter the docker again in a different shell.
Run `cd` to go to the home folder (`/home/ubuntu`).
If necessary, run `project-init.sh`
  from within the docker container.
Run `cargo run` to execute `main.rs`, or
    `cargo run --bin name` to run `file.rs`,
	where `name` is the name corresponding to `file.rs`
	in `Cargo.toml`. (Or maybe `name` is just the file's basename.)
