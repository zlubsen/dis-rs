use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-changed=assets");

    std::fs::remove_dir_all("build").unwrap_or_default();

    Command::new("bun")
        .args([
            "run",
            "tailwindcss",
            "-c",
            "tailwind.config.js",
            "-i",
            "assets/styles/index.css",
            "-o",
            "build/styles.css",
            "--minify",
        ])
        .status()
        .expect("failed to run tailwindcss");

    Command::new("bun")
        .args([
            "build",
            "--minify",
            "--outdir",
            "./build",
            "--entry-naming",
            "[name].[ext]",
            "--asset-naming",
            "[name].[ext]",
            "./assets/scripts/index.js",
        ])
        .status()
        .expect("failed to run bun");

    // std::fs::remove_file("build/index.css").unwrap_or_default();
    copy_files("templates");
}

fn copy_files(dir: &str) {
    for entry in std::fs::read_dir(dir)
        .unwrap_or_else(|err| panic!("failed to read directory `{dir}`: {err}"))
    {
        let entry = entry.expect("failed to read entry");

        if entry.file_type().unwrap().is_dir() {
            copy_files(entry.path().to_str().unwrap());
        } else {
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            let dest = format!("build/{filename}");

            std::fs::copy(path, dest).expect("failed to copy file");
        }
    }
}
