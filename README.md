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

Currently, when decoding a geohash the code in geohash::decode::decode does the following:

1. Iterate over each character in the hash string
2. Decode each character to a 5-bool tuple
3. Apply each bit (clumsily) to the working intervals, contracting them as necessary

Instead, I would much rather present an interface that goes directly from a string of base32 encoded characters to a stream of bits.  That is the following would be valid:

```rust
  let hash = ~"zzzz"
  for bit in base32::decode_bits(hash) {
    // `bit` above is a bool
    // ... do work here ...
  }
```

Given something like the above, I think you could write the decode algorithm as:

1. get the decoded bits iterator
2. create an iterator that returns lon then lat a infinite cycle
3. zip the two iterators from above together
4. iterate the zipped stream, contracting the geo component appropriately if the bit is set


## Contributing

1. Fork it
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create new Pull Request
