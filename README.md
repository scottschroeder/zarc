# zarc
Zip Archive Helper

## Motivation
There are a wide variety of compressed archives out there, and each one
has its own command line with [cryptic flags](https://xkcd.com/1168/).
Everyone seems to have a bit of
[bash](https://blog.logentries.com/2012/12/how-to-extract-any-archive/)
or [ruby](https://gist.github.com/martinus/2226) that takes care of
this for them, so why not rust?

Rust is a great language for speed, memory safety, and
[cute mascots](http://www.rustacean.net/assets/rustacean-orig-gesture.png),
but none of those are the reason I chose rust for this project.
_(ok, maybe it was the cute rustacean)_ I chose rust as an educational
exercise; using rust as a simple scripting language.

## Supported Formats
- [x] Gzipped Tarball `.tar.gz` and `.tgz`