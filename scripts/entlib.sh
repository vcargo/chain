#!/bin/bash

# Switch Source of `entlib`
# Usage:
# ./entlib.sh
# ./entlib.sh git v1.0.2

lib_name="entlib"
git_repo="ssh://git@github.com/QuantumIntelligenceChain/EntLib.git"
git_tag="v1.0.0"

is_git=false
if [ "$#" -ge 1 ] && [ "$1" == "git" ]; then
    is_git=true
    if [ "$#" -eq 2 ]; then
        git_tag=$2
    fi
fi

# Function to determine the relative path based on directory depth
get_relative_path() {
    local file_path="$1"
    local depth=$(awk -F/ '{print NF-1}' <<< "$file_path")
    local path="./"

    local parent_dir="../"
    if (( depth > 1 )); then
        path="$(printf './%.0s' $(seq 1 $((depth - 1))))$parent_dir"
    fi

    echo "$path"
}

find . -type f -name "Cargo.toml" | while read -r file; do
    # Check if 'lib_name' exists in the Cargo.toml
    if ! grep -q "^${lib_name} *= *" "$file"; then
        continue
    fi

    # Get the relative path
    relative_path=$(get_relative_path "$file")

    # Get the directory of the Cargo.toml file
    dir=$(dirname "$file")

    # Change to that directory
    cd "$dir" || continue  # Continue if cd fails

    # Add commands to execute in the Cargo.toml directory
    if [ "$is_git" == true ]; then
        cargo add $lib_name --git $git_repo --tag $git_tag --offline
    else
        cargo add $lib_name --path ".${relative_path}${lib_name}"
    fi

    # Remove version entries from lines starting with lib_name
    sed -i -E "/^${lib_name} *=/ {
        s/(version *= *\"[^\"]*\")//g;  # Remove version = "..."
        s/, *,/,/g;                     # Remove any duplicate commasRemove any extra commas
        s/\{ *,/\{/;                    # Remove comma after opening brace
        s/, *}/ }/;                     # Remove comma before closing brace
    }" "Cargo.toml"

    # Optionally: go back to the previous directory
    cd - > /dev/null  # Return to the previous directory without printing output
done
