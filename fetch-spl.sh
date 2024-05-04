#!/usr/bin/env bash
#
# Fetches the latest SPL programs and produces the svm-station-genesis command-line
# arguments needed to install them
#

set -e

# Check if download path is provided as an argument
if [ -z "$1" ]; then
  echo "Error: Please provide the download path as an argument."
  exit 1
fi

pathDir=$HOME/.svmstationd
download_path="$pathDir/$1"

upgradeableLoader=BPFLoaderUpgradeab1e11111111111111111111111

fetch_program() {
  declare name=$1
  declare version=$2
  declare address=$3
  declare loader=$4

  declare so="$download_path/spl_$name-$version.so"

  if [[ $loader == "$upgradeableLoader" ]]; then
    genesis_args+=(--upgradeable-program "$address" "$loader" "$so" none)
  else
    genesis_args+=(--bpf-program "$address" "$loader" "$so")
  fi

  if [[ -r $so ]]; then
    return
  fi

  if [[ -r "$download_path"/.cache/solana-spl/$so ]]; then
    cp "$download_path"/.cache/solana-spl/"$so" "$so"
  else
    echo "Downloading $name $version"
    so_name="spl_${name//-/_}.so"
    (
      set -x
      curl -L --retry 5 --retry-delay 2 --retry-connrefused \
        -o "$so" \
        "https://github.com/solana-labs/solana-program-library/releases/download/$name-v$version/$so_name"
    )

    mkdir -p "$(dirname "$download_path"/.cache/solana-spl/"$so")"
    cp "$so" "$download_path"/.cache/solana-spl/"$so"
  fi

}

# Create the download path directory if it doesn't exist
mkdir -p "$download_path"

fetch_program token 3.5.0 TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA BPFLoader2111111111111111111111111111111111
fetch_program token-2022 0.9.0 TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb BPFLoaderUpgradeab1e11111111111111111111111111
fetch_program memo  1.0.0 Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo BPFLoader1111111111111111111111111111111111
fetch_program memo  3.0.0 MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr BPFLoader2111111111111111111111111111111111
fetch_program associated-token-account 1.1.2 ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL BPFLoader2111111111111111111111111111111111
fetch_program feature-proposal 1.0.0 Feat1YXHhH6t1juaWF74WLcfv4XoNocjXA6sPWHNgAse BPFLoader2111111111111111111111111111111111

# Write the genesis arguments file inside the download path
echo "${genesis_args[@]}" > "$download_path/spl-genesis-args.sh"

echo
echo "Available SPL programs:"
ls -l "$download_path"/spl_*.so

echo
echo "svm-station-genesis command-line arguments (spl-genesis-args.sh):"
cat "$download_path/spl-genesis-args.sh"

rm -rf "$download_path"/.cache