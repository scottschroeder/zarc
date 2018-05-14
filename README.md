# zarc
Zip Archive Helper

## Motivation
There are a wide variety of compressed archive formats out there, and each one
has its own command line with [cryptic flags](https://xkcd.com/1168/).
Everyone seems to have thier own bit of
[bash](https://blog.logentries.com/2012/12/how-to-extract-any-archive/)
or [ruby](https://gist.github.com/martinus/2226) that takes care of
this for them, so why not rust?

I chose rust as an educational exercise; using rust as a simple scripting language.

## Supported Formats
- [x] Gzipped Tarball `.tar.gz` and `.tgz`
- [x] Bzipped Tarball `.tar.bz2` and `.tbz2`
- [x] Bzipped Archive `.bz2`
- [x] Tarballs `.tar`
- [x] XZipped files `.xz`
- [x] GZipped files `.gz` and `.Z`
- [x] Zipped files `.zip`
- [ ] Rar files `.rar` (untested)
- [ ] 7zip files `.7z` (untested)
