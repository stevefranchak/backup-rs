#!/bin/bash
# Creates a release binary and discards all symbols using 'strip'

# Contents of get_script_dir() from https://stackoverflow.com/a/246128
get_script_dir() {
  local SOURCE="${BASH_SOURCE[0]}"
  while [ -h "$SOURCE" ]; do # resolve $SOURCE until the file is no longer a symlink
    local DIR="$( cd -P "$( dirname "$SOURCE" )" && pwd )"
    SOURCE="$(readlink "$SOURCE")"
    [[ $SOURCE != /* ]] && SOURCE="$DIR/$SOURCE" # if $SOURCE was a relative symlink, we need to resolve it relative to the path where the symlink file was located
  done
  echo "$( cd -P "$( dirname "$SOURCE" )" && pwd )"
}

SCRIPT_DIR=$(get_script_dir)
PROJECT_ROOT_DIR="$SCRIPT_DIR/.."
CARGO_TOML_FILE="$PROJECT_ROOT_DIR/Cargo.toml"
TARGET_DIR="$PROJECT_ROOT_DIR/target/release"

# Check that Cargo.toml exists
if [ ! -f $CARGO_TOML_FILE ]; then
  (>&2 echo "Error: Cargo.toml does not exist in the Rust project's root directory")
  exit 1
fi

# Get the name of the binary for this Rust project
NAME_REGEX="name[ ]*=[ ]*\"([a-zA-Z0-9_]+)\""
while read line; do
  if [[ $line =~ $NAME_REGEX ]]; then
    BINARY_NAME="${BASH_REMATCH[1]}"
    break
  fi
done < $CARGO_TOML_FILE

BINARY_PATH="$TARGET_DIR/$BINARY_NAME"
cargo build --release && strip "$BINARY_PATH"
echo "Built! Binary size is "$(ls -lah $BINARY_PATH | awk -F " " '{print $5}')
