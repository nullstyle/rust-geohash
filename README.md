# Geohash

A library for decode/encoding geohashes

## Usage

```rust

use geohash::Geohash;

let hash = Geohash::decode("zz");

```

## Rough edges I don't yet know how to fix

These are the areas of the codebase where my ignorance of rust has made for what I consider shitty code.  I don't yet have the knowledge of _how_ to fix the issues specifically, but I can describe how I would want to fix them.

### Base32 decoding

Currently, the base32 module only operates on single characters, but I would rather it could work on both characters and strings such that both the following statements work:

```rust
let bits_from_char = base32.decode_vec('u');
let bits_from_vec  = base32.decode_vec("uuu");
```

## Contributing

1. Fork it
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create new Pull Request
