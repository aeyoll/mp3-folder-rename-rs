# MP3 folder rename

A small utility to rename mp3 folders the way _I_ like it, i.e.:

> Artist - Year - Album

Usage
---

```shell script
mp3-folder-rename [FLAGS] <SRC>...
```

#### Flags:

```
-d, --debug      Debug mode
-n, --dry-run    Dry run mode
-h, --help       Prints help information
-V, --version    Prints version information
```

#### Args:

```
<SRC>...    The source folders to rename
```

Development
---

```shell script
git clone git@github.com:aeyoll/mp3-folder-rename-rs.git
cd mp3-folder-rename-rs
cargo run -- 
```