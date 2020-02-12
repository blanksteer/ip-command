use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    if let Some(clang) = executable_path("clang") {
        let output = Command::new(&clang)
            .arg("-target")
            .arg("bpf")
            .arg("-O2")
            .arg("-c")
            .arg("src/command/test_fixture/xdp_test.c")
            .arg("-o")
            .arg("src/command/test_fixture/xdp_test.o")
            .output()
            .expect("failed to compile xdp test");
        if !output.status.success() {
            let error = String::from_utf8(output.stderr).unwrap();
            panic!("{}", error)
        }
    }
}

fn executable_path(name: &str) -> Option<PathBuf> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .filter_map(|dir| {
                let full_path = dir.join(name);
                if full_path.is_file() {
                    Some(full_path)
                } else {
                    None
                }
            })
            .next()
    })
}
