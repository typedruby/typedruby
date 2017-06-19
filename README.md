# Installation

## OS X

```sh
brew install bison ragel rust
echo 'export PATH="/usr/local/opt/bison/bin:$PATH"' >> ~/.bash_profile
# restart shell to get new $PATH
cargo build

# Then run it with:
target/debug/typedruby your/ruby/file.rb
```
