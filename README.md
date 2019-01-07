compressed_log_sink
===

A WebSocket server that accepts compressed log buffers streamed by [compressed_log](https://crates.io/crates/compressed_log) crate.

# Usage

```
Compressed log sink.

Usage:
  compressed_log_sink [ --bind=<address> ] [ --output=<stream> ]
  compressed_log_sink (-h | --help)
  compressed_log_sink --version

Options:
  -h --help     Show this screen.
  --version     Show version.
  --bind=<address>  Bind to address [default: 0.0.0.0:9999].
  --output=<stream>  Output stream [default: -].
```

Options:

- `--bind` - Specify an address to bind the HTTP server to. By default it uses `0.0.0.0:9999`.
- `--output` - Specify an output stream. A special file name `-` means the server will always write logs to stdout, otherwise a file is created in append mode.
