# Git commands

## Client Server exchange

### Create a simple server

A simple git server can be started by using the following command
in the parent directory of one or more git repositories:

```sh
git daemon --reuseaddr --verbose  --base-path=. --export-all
```

Now access any of the repository on the server.

```sh
export GIT_TRACE_PACKET=1
git ls-remote git://127.0.0.1/myrepo
```

#### Debug options

GIT_TRACE_PACKET - show packet line information
GIT_TRACE - show general command execution debug information
GIT_CURL_VERBOSE - show curl debug information when using the http transport
GIT_DEBUG_SEND_PACK - enable debug output in upload-pack
GIT_TRANSPORT_HELPER_DEBUG - enables debug output for the remote helpers

