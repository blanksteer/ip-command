# ip-command

A Rust wrapper around the Linux ip(8) command. Powered by Tokio with full async/await support.

## Why?

* Rust support for netlink is immature.
* Why should my application communicate directly with the kernel?
* Ubiquitous and available on all platforms, no need to ship a binary.
* Not really expected to be a performance bottleneck, if so, you are probably too chatty with the kernel.
* No dependencies on kernel datastructures, increased portability.

## Status

The iproute2 tools have a vast collection of knobs and dials, this project while providing a skeleton does not yet
support every feature from the underlying tools. Over time this coverage is expected to improve.

### Feature Coverage

- [x] Network devices (link).
- [ ] Protocol address of device (addr).
- [ ] Address labels (addrlabel).
- [ ] Routing table entries (route).
- [ ] Routing policy rules (rule).
- [ ] ARP or NDISC cache entries (neigh).
- [ ] Neighbor cache's operations (ntable).
- [ ] Tunnel over IP (tunnel).
- [ ] Manage TUN/TAP devices (tuntap).
- [ ] Multicast addresses (maddress).
- [ ] Multicast routing cache entries (mroute).
- [ ] Multicast routing policy rules (mrule).
- [ ] Monitor netlink messages (monitor).
- [ ] Manage IPSec policies (xfrm).
- [x] Manage network namespaces (netns).
- [ ] Tunnel ethernet over IP (l2tp).
- [ ] Manage TCP Metrics (tcp_metrics).
- [ ] Manage tokenized interface identifiers (token).
- [ ] MACsec device configuration (macsec).

### Upcoming Features

* Protocol addresses.