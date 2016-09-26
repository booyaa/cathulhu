# Cathulhu

> In her house at San Francisco, dead Cathulhu lies dreaming.

Cathulhu is a GitHub cli written in Rust.

# Installation

```
cargo install cathulhu
```

# Usage

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
