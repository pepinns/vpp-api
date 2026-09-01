extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn find_vpp_lib_dir() -> String {
    /*
     * In the future there's more cleverness possibly to be added.
     * For now this will do.
     */
    "/usr/lib/x86_64-linux-gnu/".to_string()
}

fn find_vpp_include_dir() -> String {
    /*
     * A typical VPP -dev apt package drops VPP's headers under the
     * arch-independent /usr/include, unlike the arch-specific lib dir
     * above. In the future there's more cleverness possibly to be added.
     * For now this will do.
     */
    "/usr/include".to_string()
}

fn git_version() -> String {
    use std::process::Command;

    let describe_output = Command::new("git")
        .arg("describe")
        .arg("--all")
        .arg("--long")
        .output()
        .unwrap();

    let mut describe = String::from_utf8_lossy(&describe_output.stdout).to_string();
    describe.pop();
    describe
}

fn main() {
    println!("cargo:rustc-env=GIT_VERSION=version {}", git_version());
    println!("cargo:rerun-if-env-changed=VPP_LIB_DIR");
    println!("cargo:rerun-if-env-changed=VPP_INCLUDE_DIR");

    let vpp_lib_dir = match env::var("VPP_LIB_DIR") {
        Ok(val) => val,
        Err(_e) => find_vpp_lib_dir(),
    };
    if !std::path::Path::new(&format!("{}/libvppapiclient.so", vpp_lib_dir)).exists() {
        panic!("Can not find libvppapiclient.so at {}, please install python3-vpp-api or define VPP_LIB_DIR accordingly", vpp_lib_dir)
    }
    let flags = format!("cargo:rustc-flags=-L{} -lvppapiclient", vpp_lib_dir);

    // Tell cargo to tell rustc to link the VPP client library
    println!("{}", flags);

    let vpp_include_dir = match env::var("VPP_INCLUDE_DIR") {
        Ok(val) => val,
        Err(_e) => find_vpp_include_dir(),
    };
    if !std::path::Path::new(&format!("{}/vpp-api/client/stat_client.h", vpp_include_dir)).exists()
    {
        panic!(
            "Can not find vpp-api/client/stat_client.h under {}, please install a VPP -dev \
             package providing VPP's headers or define VPP_INCLUDE_DIR accordingly",
            vpp_include_dir
        )
    }

    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .clang_arg(format!("-I{}", vpp_include_dir))
        // Keep the generated bindings scoped to just the stat-segment client
        // API this crate uses - stat_client.h's transitive includes pull in
        // a large chunk of VPP's header surface that we don't want mirrored
        // into bindings.rs.
        .allowlist_function("stat_segment_connect")
        .allowlist_function("stat_segment_disconnect")
        .allowlist_function("stat_segment_ls")
        .allowlist_function("stat_segment_dump")
        .allowlist_function("stat_segment_data_free")
        .allowlist_function("stat_segment_vec_len")
        .allowlist_function("stat_segment_vec_free")
        .allowlist_function("stat_segment_string_vector")
        .allowlist_type("stat_segment_data_t")
        .allowlist_type("stat_directory_type_t")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_file_name = out_path.join("bindings.rs");
    bindings
        .write_to_file(out_file_name.clone())
        .expect("Couldn't write bindings!");
}
