# rbs_rust_parser
parse ruby rbs files using the pest parser in rust

# why
this will be the base for parsing the files for a rbs language server and other tools.
The stblib parsing is complete but it is very much a work in progress.

As of Aug 25 2020 all 80 stdlib files parse. [1]
As of Aug 26 2021 all 6 stdlib files fail to parse. I am not looking in to fixing this at the moment.

update and run test: 
```
cd test/stdlib;
git pull;
cd../..;
cargo test;
```


[1] pending this pr https://github.com/ruby/rbs/pull/381 for a post line comment fix
