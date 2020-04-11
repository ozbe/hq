# hq

Extract html based on css selector.

## Examples

### File

```
 cargo run -- "div > h1" example.html
```

### Pipe (Curl)
```
curl -sL -H "User-Agent: hq" http://example.com | cargo run -- "div > h1"
```