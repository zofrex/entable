# Entable

Entable parses ascii tables and reformats them, fixing broken padding and spacing to make them look pretty.

It takes this:

```
+-------------+---------+-----------+
| VM | status | role |
+-------------+---------+-----------+
| spider | ok | webserver |
| grasshopper | stopped | db |
+-------------+---------+-----------+
```

And turns it into this:

```
+-------------+---------+-----------+
| VM          | status  | role      |
+-------------+---------+-----------+
| spider      | ok      | webserver |
| grasshopper | stopped | db        |
+-------------+---------+-----------+
```

## Installation

### Homebrew

```
brew tap zofrex/entable https://github.com/zofrex/entable
brew install entable
```

### Other platforms

Check out the repositority and build with `cargo build`. Install with `cargo install`, or alternatively place the binary anywhere on your path.

## Usage

Takes data on stdin, outputs on stdout, has no parameters.