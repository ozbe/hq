# hq

![Verify](https://github.com/ozbe/hq/workflows/Verify/badge.svg)

Extract html based on CSS selectors.

A utility for selecting HTML fragments from an HTML document or fragment
with a CSS selector group.

## Usage

A platform specific build of hq can be run in the corresponding
platform's command line interface. Run hq with `cargo run -- -h` or
`cargo run -- --help` to view the latest* available flags, arguments and
commands.

Here is an example of the long help output (`--help`):

```text
hq 0.1.0
Extract html based on CSS selectors.

A utility for selecting HTML fragments from an HTML document or fragment with a CSS selector group.

USAGE:
    hq <selectors> [file]

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information


ARGS:
    <selectors>    
            CSS selector group
            
            Group of CSS Selectors used to select HTML fragments from input HTML.
    <file>         
            HTML file
            
            Local file path to a file containing a valid HTML document or fragment. If file argument missing, hq reads
            from standard input.
```

## Examples

### File

```
 cargo run -- "div > h1" example.html
```

### Pipe (Curl)

```
curl -sL -H "User-Agent: hq" http://example.com | cargo run -- "div > h1"
```

## Releases

### Create Release

To create a new release, the only manual part of the process is creating
and pushing a tag to GitHub. The following commands will create and push
a tag. Before running the commands, be sure to update `MAJOR`, `MINOR`,
and `PATCH` based on the current version and
[Semantic Version](https://semver.org/) guidelines.

**Create and Push Tag**
```bash
$ export MAJOR=0 MINOR=1 PATCH=0
$ git checkout master
$ git pull
$ git tag -a v$MAJOR.$MINOR.$PATCH
# enter tag message
$ git push origin v$MAJOR.$MINOR.$PATCH
```

After pushing a new tag, the
[Publish Workflow](.github/workflows/publish.yml) will create a
corresponding GitHub Release and attach artifacts for each supported
platform.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms
or conditions.
