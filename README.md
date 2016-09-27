# Cathulhu

> In her house at San Francisco, dead Cathulhu lies dreaming.

Cathulhu is a GitHub cli written in Rust.

# Installation

```
cargo install cathulhu
```

# Usage

You need a GitHub Personal Access Token. You can generate one [here][1] (you
only need to give access to public repos). Store your token in an environment
variable called `CATHULHU_GH_PAT`. You can add these to your `.bashrc` by
entering the following:


`export CATHULHU_GH_PAT=keep_it_safe_keep_it_secret`

There's only one API endpoint(issues) and flag (list) implemented:

```
Issue related commands

USAGE:
    cathulhu issues [FLAGS] <repository name>

FLAGS:
    -h, --help       Prints help information
    -l, --list       lists open issues by number, title and url (pipe delimited)
    -V, --version    Prints version information


ARGS:
    <repository name>    Set repository name
```

Copyright 2016 Mark Sta Ana.

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/licenses/LICENSE-2.0> at your option. This file may not
be copied, modified, or distributed except according to those terms.

[1]:https://github.com/settings/tokens
