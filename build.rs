use std::process::Command;

fn main() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&[
                "/C",
                "esbuild",
                "--outdir=dist",
                "--format=esm",
                "--bundle",
                "tauri/tooling/api/src/app.ts",
                "tauri/tooling/api/src/clipboard.ts",
                "tauri/tooling/api/src/tauri.ts",
                "tauri/tooling/api/src/event.ts",
                "tauri/tooling/api/src/mocks.ts",
            ])
            .output()
            .unwrap();
    } else {
        Command::new("esbuild")
            .args(&[
                "--outdir=dist",
                "--format=esm",
                "--bundle",
                "tauri/tooling/api/src/app.ts",
                "tauri/tooling/api/src/clipboard.ts",
                "tauri/tooling/api/src/tauri.ts",
                "tauri/tooling/api/src/event.ts",
                "tauri/tooling/api/src/mocks.ts",
            ])
            .output()
            .unwrap();
    };
}
