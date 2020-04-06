# hq

Extract html based on css selector.

## Example

### File

```
 cargo run -- "div > h1" < example.html
```

### Curl
```
curl -sL -H "User-Agent: hq" http://example.com | cargo run -- "div > h1"
```