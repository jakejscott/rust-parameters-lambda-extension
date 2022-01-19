# Rust parameters lambda extension

# Research

https://github.com/aws-samples/aws-lambda-extensions
https://github.com/aws-samples/aws-lambda-extensions/blob/main/go-example-extension/Makefile

Lambda extensions deep dive
https://serverlessland.com/learn/lambda-extensions

# Installing

```sh
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Or update rust to the latest version
# rustup update

# Add target so we can cross compile for x86 Lambdas
rustup target add x86_64-unknown-linux-musl

# Install dependencies
sudo apt-get update
sudo apt-get upgrade
sudo apt-get install zip musl-tools -y
```

# Building

```sh
./build.sh
```

# Deploying

```sh
cd cdk
cdk bootstrap
cdk deploy
```
