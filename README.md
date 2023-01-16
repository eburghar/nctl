# nctl

Command line tool to manage files on a WebDAV server (originally for Nextcloud). It is mainly
used for copying backup files from containers to the server and ensure we have a fixed size
number of consecutive backups by removing old ones.

TODO:

- recursive ls
- delete folder
- recursive copy of local folder
- copy from webdav to local file

```
nctl 0.5.0

Usage: nctl [-c <config>] [-v] [-d] <command> [<args>]

Interact with a webdav server (Nextcloud)

Options:
  -c, --config      configuration file containing webdav connection parameters
                    and defining paths
  -v, --verbose     more detailed output
  -d, --dry-run     simulate but don't do anything
  --help            display usage information

Commands:
  cp                Copy a local file to a webdav folder
  ls                List a webdav folder content
  rm                Delete files from a webdav server
  cleanup           Delete oldest files matching an expression from a webdav
                    server
```

## Configuration

The configuration is a simple yaml file

```yaml
# host parameters and credentials
account:
  url: https://drive.example.com
  prefix: /remote.php/dav/files/backup
  user: backup
  password: "mypassword"

# shorcuts for not having to type full relative paths on command line
paths:
  mydb1:
    base: "/db/mydb1"
    # optional cleanup config to keep the nth most recent files
    # matching a given regex (ie: removing the others)
    cleanup:
      dump:
        regex: mydb1_.*\.dump
        keep: 5

  mydb2:
    base: "/db/mydb2"
    cleanup:
      dump:
        regex: mydb2_.*\.dump
        keep: 10
```

## Copy

`dst` must follow the syntax `path_key:relative_file_path`. In case `relative_file_path` is
empty or ends with `/`, the `src` base name is taken.

```
nctl 0.5.0

Usage: nctl cp <src> <dst>

Copy a local file to a webdav folder

Positional Arguments:
  src               source
  dst               destination

Options:
  --help            display usage information
```
  

```bash
# keep the same filename
nctl cp mydb1_xxx_yyy.dump mydb1:
# change the destination filename
nctl cp mydb1_xxx_yyy.dump mydb1:mydb1_xxx_zzz.dump
```

## List

Arguments must follow the syntax `path_key:relative_dir_path`.

```
nctl 0.5.0

Usage: nctl ls <path>

List a webdav folder content

Positional Arguments:
  path              path to list files from

Options:
  --help            display usage information
```

```bash
# ls the base directory
nctl ls mydb1: 
# ls a subdirectory
nctl ls mydb1:sub/path
```

## Delete

Arguments must follow the syntax `path_key:relative_file_path`.

```
nctl 0.5.0

Usage: nctl rm [<files...>]

Delete files from a webdav server

Positional Arguments:
  files             files to delete

Options:
  --help            display usage information
```

```bash
nctl rm mydb1:mydb1_xxx_yyy.dump
```

## Cleanup

Arguments must follow the syntax `path_key:cleanup_key`.

```
nctl 0.5.0

Usage: nctl cleanup [<paths...>]

Delete oldest files matching an expression from a webdav folder

Positional Arguments:
  paths             cleanup configs to use

Options:
  --help            display usage information
```

```bash
nctl cleanup mydb1:dump mydb2:dump
```