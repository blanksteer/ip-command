# ip-command

A Rust wrapper around the Linux ip(8) command. Powered by Tokio with full async/await support.

## Why?

* Rust support for netlink is immature.
* Why should my application communicate directly with the kernel?
* Ubiquitous and available on all platforms, no need to ship a binary.
* Not really expected to be a performance bottleneck, if so, you are probably too chatty with the kernel.
* No dependencies on kernel datastructures, increased portability.
