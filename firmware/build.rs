fn main() {
	// By default, Cargo will re-run a build script whenever
	// any file in the project changes. By specifying `memory.x`
	// here, we ensure the build script is only re-run when
	// `memory.x` is changed.
	println!("cargo:rerun-if-changed=linker_scripts/");

	use std::{env, fs, path};
	let out = path::PathBuf::from(env::var("OUT_DIR").unwrap());
	fs::write(
		out.join("memory.x"),
		include_bytes!("linker_scripts/memory.x.nrf52840"),
	)
	.unwrap();
	// println!("cargo:rustc-link-arg=-Tmemory.x.nrf52840");
	println!("cargo:rustc-link-search={}", out.display());
}
