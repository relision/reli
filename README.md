# relision
[![Build Status](https://travis-ci.org/relision/reli.svg?branch=master)](https://travis-ci.org/relision/reli)
[![Coverage Status](https://coveralls.io/repos/daheise/reli/badge.svg)](https://coveralls.io/r/daheise/reli)

A Rust library and REPL for the Elision term rewriter.  This is being developed in [Rust][rust].  The project uses [Semantic Versioning][semantic-version], and is released under the two-clause BSD license (see the License section below).


## Acknowledgements

This project uses [TOML][toml] (via [toml-rs][]) for markup and [linenoise][] (via [rust-linenoise][]) for line parsing.  Arbitrary precision integers are provided by [Num][num].


## License
<pre>
Copyright (c) 2015, Stacy Prowell
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

* Redistributions of source code must retain the above copyright notice, this
  list of conditions and the following disclaimer.

* Redistributions in binary form must reproduce the above copyright notice,
  this list of conditions and the following disclaimer in the documentation
  and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
</pre>


[toml]:				https://github.com/toml-lang/toml
[toml-rs]:			https://github.com/alexcrichton/toml-rs
[rust]:				http://www.rust-lang.org
[num]:				https://github.com/rust-lang/num
[linenoise]:		https://github.com/antirez/linenoise
[rust-linenoise]:	https://github.com/octplane/rust-linenoise
[semantic-version]:	http://semver.org
