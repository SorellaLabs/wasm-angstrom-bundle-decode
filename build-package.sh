#!/bin/bash

# Exit on any error
set -e

# Check if rs-bundle-decode directory exists
if [ ! -d "rs-bundle-decode" ]; then
    echo "Error: rs-bundle-decode directory does not exist"
    exit 1
fi

# Build the wasm package
echo "Building wasm package..."
wasm-pack build --target web rs-bundle-decode

# Clean up and prepare pkg directory
echo "Preparing pkg directory..."
rm -rf pkg
cp -r rs-bundle-decode/pkg pkg
rm -f pkg/.gitignore

# Read current version from version file
if [ ! -f "version" ]; then
    echo "Error: version file does not exist"
    exit 1
fi

current_version=$(cat version | tr -d '\n' | tr -d ' ')
echo "Current version: $current_version"

# Increment version by 0.0.1
IFS='.' read -ra VERSION_PARTS <<< "$current_version"
major="${VERSION_PARTS[0]:-0}"
minor="${VERSION_PARTS[1]:-0}"
patch="${VERSION_PARTS[2]:-0}"

# Validate version parts are numbers
if ! [[ "$major" =~ ^[0-9]+$ ]] || ! [[ "$minor" =~ ^[0-9]+$ ]] || ! [[ "$patch" =~ ^[0-9]+$ ]]; then
    echo "Error: Invalid version format in version file. Expected format: X.Y.Z"
    exit 1
fi

# Increment patch version
patch=$((patch + 1))
new_version="$major.$minor.$patch"
echo "New version: $new_version"

# Update version file
echo "$new_version" > version

# Update version in pkg/package.json using a more reliable method
# Create a temporary file with the updated content
temp_file=$(mktemp)

# Read the package.json and update version
node -e "
const fs = require('fs');
const pkg = JSON.parse(fs.readFileSync('pkg/package.json', 'utf8'));
pkg.name = '@sorellalabs/angstrom-bundle-decode';
pkg.version = '$new_version';
pkg.repository = {
    type: 'git',
    url: 'https://github.com/SorellaLabs/wasm-angstrom-bundle-decode.git'
};
fs.writeFileSync('$temp_file', JSON.stringify(pkg, null, 2));
"

# Move the temporary file to replace package.json
mv "$temp_file" pkg/package.json

echo "Build complete! Package version updated to $new_version"

# Verify npm authentication
echo "Checking npm authentication..."
if ! npm whoami > /dev/null 2>&1; then
    echo "Error: Not logged into npm. Please run 'npm login' first."
    exit 1
fi

# Update npm version in package.json (this also creates a git tag)
echo "Setting npm version..."
cd pkg
npm version $new_version
cd ..

# Publish to npm
echo "Publishing to npm..."
cd pkg
npm publish --scope=@sorellalabs --access=public
cd ..

echo "Successfully published version $new_version!"