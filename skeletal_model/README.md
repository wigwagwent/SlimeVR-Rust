# 🦀 Ferrous SimeVR Skeletal Model
A WIP implementation of a new full-body tracking skeletal model & solver, for use
in Virtual Reality.

Written in Rust, callable from Java and Typescript/Node.js.


> This is *not* the official skeletal model for SlimeVR, that model is
> [here][official model], intertwined deep in the official SlimeVR Server
> codebase. This alternative implementation seeks to improve on the official one.

## 📝 How does it work?

Its worth reading the documentation [here][docs], lots of time has been spent to
explain how it works.


## 🔭 Goals
* Written in Rust, callable by all[^1] languages.
* Single purpose, *just* the math. Leave other tasks like networking and IO up to the
  user of the library.
* Run on all platforms, including Windows, Mac, Linux, Oculus Quest, Steam Deck,
  WebAssembly etc.
* High performance, no language runtime or JVM, embed it in your program as a native
  library.
* Help others build on top of our work, including non-rust developers. We want to
  spread affordable full-body tracking to as many people as possible.
* Extensive [documentation][docs], easily understood by non-rustaceans. This will
  ensure that even non-experts can help contribute and improve things for the community.
* Leverage good [math][nalgebra] and [graph][daggy] libraries to simplify implementation.


## 🚧 Implementation Status
We are under active development. If you love Rust, math, or full-body tracking, please reach
out and help make this a reality!

- [X] Graph datastructure and associated data implemented.
- [X] Everything well documented.
- [X] Supports basic positional constraints (Vive trackers).
- [X] Supports rotational constraints (SlimeVR trackers).
- [ ] Align tracker inputs (IMU yaw alignment).
- [ ] Implement solver to turn the constraints into the estimated skeleton pose.
- [ ] Provide TypeScript/Node.js bindings.
- [ ] Validate that the library works by using it in a Typescript or Rust implementation of the SlimeVR server.
- [ ] Provide Java bindings.
- [ ] Integration with official SlimeVR [Server][java server]. [^2]


[docs]: https://slimevr.github.io/SlimeVR-Rust/skeletal_model/
[official model]: https://github.com/SlimeVR/SlimeVR-Server/blob/8adf3fe5912481f3f1d8658f917292617bd308dc/src/main/java/dev/slimevr/vr/processor/skeleton/HumanSkeleton.java
[daggy]: https://docs.rs/daggy/latest/daggy/
[nalgebra]: https://nalgebra.org/
[java server]: https://github.com/SlimeVR/SlimeVR-Server

[^1]: This is *not* the official skeletal model for SlimeVR, that model is actually 
[here][official model] intertwined deep in the official SlimeVR Server codebase. This
alternative implementation seeks to improve on the official one and experiment with some
new concepts

[^1]: For now, "all" means Typescript, and then later, Java. If you have a language you are
passionate about adding support for, just file an issue! If it can talk to C, it can talk to
Rust.

[^2]: This is both unlikely and also not strictly necessary. Unlikely because 
[Eiren](https://github.com/Eirenliel), the creator slime behind SlimeVR, has stated that
they have no interest in rewriting the skeletal model in Rust or making it use anything other
than Java. There may still be a chance that this can be integrated into the official server
behind some compilation/feature flags. It might instead get ported by others to Java. Either
this library adds some features that don't exist in the official model, and has a different
tech stack and goals. We plan on making a TypeScript version of the SlimeVR server regardless.