#!/bin/bash

# Define the list of step numbers to ignore
ignore_steps=("3")

# Iterate over all folders in the 'tutorial' directory
for dir in tutorial/*/
do
    dir=${dir%/}  # Remove the trailing slash from the directory name
    dir=${dir#tutorial/}  # Remove the 'tutorial/' prefix from the directory name

    # If the folder name starts with 'step-' and contains a subfolder named 'runtime'
    if [[ $dir == step-* && -d "tutorial/${dir}/runtime" ]]
    then
        # Extract the step number
        step_number=${dir#step-}

        # Check if the step number is in ignore_steps
        if ! printf '%s\n' "${ignore_steps[@]}" | grep -q -w "$step_number"
        then
            # Print a string
            echo "Checking $dir"

            # Create the new runtime value
            new_runtime_value="runtime = { package = \"${dir}-runtime\", path = \"../${dir}/runtime\" }"

            # Replace the value of 'runtime' in the Cargo.toml file
            sed -i '' "s#^runtime = .*#$new_runtime_value#" tutorial/node/Cargo.toml

            if cargo test -p tutorial-node >/dev/null 2>&1; then
                echo "✅ cargo test for ${dir} completed successfully"
            else
                cargo test -p tutorial-node >/dev/null
            fi

        fi
    fi
done
