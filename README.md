# urlfrontier-client

A command line client for [URL Frontier](https://github.com/crawler-commons/url-frontier), written in Rust.

URL Frontier is a gRPC API, with a reference service implementation, for the component of a web crawler that
stores the URLs and decides which ones to fetch next. `uf` talks to a Frontier from the terminal: inspect its
crawls, queues and URLs, inject URLs from a file, export them, pause the Frontier or delete data. It covers
the same operations as the Java client which comes with URL Frontier and reads and writes the same JSON
format, so files can go from one client to the other.

It is built from the API definition of URL Frontier 2.6 (`urlfrontier.proto`).

## Installation

Each [release](https://github.com/DigitalPebble/urlfrontier-client/releases) comes with prebuilt binaries for
Linux (x86_64, arm64), macOS (x86_64, arm64) and Windows (x86_64), each with a SHA-256 checksum. Extract
`uf` from the archive for your platform and put it on your `PATH`.

### From source

You need a [Rust toolchain](https://rustup.rs/) and `protoc`, the Protocol Buffers compiler, version 3.15 or
later, which generates the gRPC code at build time:

```
sudo apt-get install protobuf-compiler   # Debian / Ubuntu
brew install protobuf                    # macOS
```

If `protoc` is installed somewhere else, point to it with the `PROTOC` environment variable.

Then, from a clone of this repository:

```
cargo install --path .
```

This builds the `uf` binary and copies it to `~/.cargo/bin`, which `rustup` puts on your `PATH`. Run the same
command again to update it, and `cargo uninstall urlfrontier-client` to remove it.

### Shell completion

`uf completions <SHELL>` prints a script which completes the commands, their options and the values these
accept, for `bash`, `zsh`, `fish`, `elvish` or `powershell`. For bash:

```
mkdir -p ~/.local/share/bash-completion/completions
uf completions bash > ~/.local/share/bash-completion/completions/uf
```

For zsh, write it as `_uf` in a directory of your `$fpath`; for fish, as `~/.config/fish/completions/uf.fish`.
Generate it again after updating `uf`, as commands and options may have changed. The script takes effect in
new shells.

## Usage

```
uf [--host HOST] [-p PORT] <COMMAND> [OPTIONS]
```

The Frontier is reached on `localhost:7071` unless `--host` or `-p` say otherwise; these two options go
before the command. `uf --help` lists the commands and `uf <COMMAND> --help` describes their options.

Most commands apply to the crawl `DEFAULT` unless one is given with `-c` / `--crawl-id`. In a cluster, the
commands which support `-l` / `--local` aggregate over every node unless it is set, in which case only the
node `uf` is connected to is concerned.

### Commands

| Command | What it does |
|---|---|
| `get-stats` | Number of URLs, URLs in process, queues and the custom counts of the Frontier, for a crawl or one queue (`-k`) |
| `list-nodes` | Nodes forming the cluster |
| `list-crawls` | Crawls in the Frontier |
| `list-queues` | Queues of a crawl, active ones only unless `-i`; a page at a time (`-n`, `-s`), or all of them into a file with `-o` |
| `list-urls` | URLs in the Frontier with their refetch date, a page at a time; filter with `-f` (`-i` to ignore case), output as JSON with `-j`, dates in local time with `-p` |
| `count-urls` | Number of URLs, for a crawl or one queue, optionally matching a filter |
| `get-url-status` | Refetch date of a URL (`-u`); exits with code 1 if the URL is not in the Frontier |
| `get-urls` | Takes URLs due for fetching, as the crawler would, and prints them as JSON; they are not handed out again until the delay (`-d`) has elapsed |
| `put-urls` | Sends the URLs of a file to the Frontier, see below |
| `dump-urls` | Exports the URLs as JSON, see below |
| `set-active` / `get-active` | Pauses (`set-active false`) or resumes (`set-active`) the Frontier, or checks its state. A paused Frontier still accepts URLs but does not hand any out |
| `set-crawl-limit` | Stops a queue (`-k`) from handing out URLs once this many of them (`-l`) have been completed; 0 removes the limit |
| `set-log-level` | Log level of a Java package in the Frontier service |
| `delete-queue` | Deletes a queue and its URLs |
| `delete-crawl` | Deletes a crawl and its URLs |
| `purge-urls` | Deletes the URLs stored more than a number of days ago (`-d`) |
| `completions` | Shell completion script, see [Shell completion](#shell-completion) |

### Examples

```
# how big is the crawl?
uf get-stats
uf --host frontier.internal -p 7071 get-stats -c news

# the 10 first URLs of a queue, with readable dates
uf list-urls -k example.com -n 10 -p

# how many URLs contain "sitemap"?
uf count-urls -f sitemap -i

# pause the Frontier, then resume it
uf set-active false
uf set-active
```

## Injecting URLs

```
uf put-urls -f urls.txt.gz -t 4 -b 100
```

`put-urls` reads a file line by line. Files whose name ends in `.gz` are decompressed. Each line is either:

- a plain URL, which is sent as a newly discovered URL to the crawl given with `-c`. The Frontier creates it
  unless it already knows it, and decides which queue it goes to (by default, its hostname);
- a URL item in JSON, which gives full control over the queue, crawl, metadata and refetch date. A discovered
  URL is created if it is not known yet; a known URL is created or updated:

```
{"discovered":{"info":{"url":"https://example.com/","key":"example.com","metadata":{"lang":{"values":["en"]}}}}}
{"known":{"info":{"url":"https://example.com/about","key":"example.com"},"refetchableFromDate":"1791615712"}}
```

A known URL without a refetch date is considered done and won't be fetched again. Dates are in seconds since
the Unix epoch. The two kinds of lines can be mixed in the same file. Blank lines are ignored and invalid ones
reported with their line number.

The Frontier confirms each URL with a status: `OK`, `SKIPPED` (e.g. a discovered URL it already knows) or
`FAIL`. The throughput is displayed every 30 seconds, then a summary of the statuses at the end. The exit
code is 1 if the file could not be read or the connection failed.

Options to make it faster:

- `-t`: number of streams sending URLs, each on its own connection. The file is read once and its lines shared
  between them.
- `-w`: number of URLs a stream can send before they have been confirmed (10000 by default).
- `-b`: sends the discovered URLs in batches of this size, which amortises the cost of each message. Known
  URLs are always sent one by one. If the Frontier is too old to accept batches, `uf` falls back to sending
  every URL individually.

## Exporting URLs

```
uf dump-urls -o frontier.ndjson.gz -t 4
```

`dump-urls` writes every URL, one JSON line each, in the format read by `put-urls`; this is how you migrate a
Frontier to a different backend or version. The output is compressed if the file name ends in `.gz`, and goes
to the standard output if there is no `-o`; progress messages go to the standard error.

All the crawls are exported unless one is given with `-c`. With `-t` greater than 1, the queues are listed
first and exported in parallel on that many connections.

The dump only covers the node `uf` is connected to: in a cluster, dump every node and concatenate the files.
Pause the Frontier and stop sending URLs to it first if the dump must be complete.

## Releasing

Set the new version in `Cargo.toml`, commit, then push a tag with the same version, e.g.
`git tag 0.2.0 && git push origin 0.2.0`. The release workflow creates the GitHub release with notes generated
from the merged pull requests and attaches the binaries to it.

## License

Apache License 2.0, see [LICENSE](LICENSE).
