# nctl

Command line tool to manage files on a WebDAV server (originally for Nextcloud). It is mainly
used for copying backup files from containers to the server and ensure we have a fixed size
number of consecutive backups by removing old ones.

```
Usage: nctl [-c <config>] [-v] <command> [<args>]

Extract latest projects archives from a gitlab server

Options:
  -c, --config      configuration file containing projects and gitlab connection
                    parameters
  -v, --verbose     more detailed output
  --help            display usage information

Commands:
  cp                Copy a file to/from a webdav folder
  ls                List a webdav folder content
  rm                Rm files from a webdav server
```