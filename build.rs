use std::path::{PathBuf};

fn main() {
	generate_bindings();
}

fn generate_bindings() {
	let is_static = cfg!(feature = "static");

	let arch = std::env::var_os("CARGO_CFG_TARGET_ARCH")
		.expect("Failed to get target architecture!");
	let os = std::env::var_os("CARGO_CFG_TARGET_OS")
		.expect("Failed to get target operating system!");
	assert_eq!(os, "windows");

	let mut sdk_path: PathBuf = std::env::var_os("CARGO_MANIFEST_DIR")
		.expect("Failed to get manifest directory!").into();
	sdk_path.push("sdk");

	let mut lib_path = sdk_path.join("lib");
	lib_path.push(arch);

	let inc_path = sdk_path.join("inc");
	let header_path = sdk_path.join(if is_static { "wrapper-static.hpp" } else { "wrapper.hpp" });

	println!("cargo:rerun-if-changed={}", header_path.display());
	println!("cargo:rustc-link-search={}", lib_path.display());

	if is_static {
		println!("cargo:rustc-link-lib=static=vJoyInterfaceStat");
		println!("cargo:rustc-link-lib=User32");
	} else {
		println!("cargo:rustc-link-lib=dylib=vJoyInterface");
	}

	let bindings = bindgen::builder()
		.allowlist_file(r".*\bgen-versioninfo\.h")
		.allowlist_file(r".*\bpublic\.h")
		.allowlist_file(r".*\bvjoyinterface\.h")
        .disable_name_namespacing()
		.header(format!("{}", header_path.display()))
		.clang_arg(format!("-I{}", inc_path.display()))
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()
		.expect("Failed to generate vJoyInterface bindings");

	let mut out_path = std::env::var_os("OUT_DIR")
		.map(PathBuf::from)
		.expect("Failed to get output directory!");
	out_path.push("vJoyInterface.rs");

	bindings
		.write_to_file(out_path)
		.expect("Failed to write bindings");
}
