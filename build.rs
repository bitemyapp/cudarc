use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CUDA_ROOT");
    println!("cargo:rerun-if-env-changed=CUDA_PATH");
    println!("cargo:rerun-if-env-changed=CUDA_TOOLKIT_ROOT_DIR");

    #[cfg(not(any(
        feature = "cuda-version-from-build-system",
        feature = "cuda-12030",
        feature = "cuda-12020",
        feature = "cuda-12010",
        feature = "cuda-12000",
        feature = "cuda-11080",
        feature = "cuda-11070",
    )))]
    compile_error!("Must specify one of the following features: [cuda-version-from-build-system, cuda-12030, cuda-12020, cuda-12010, cuda-12000, cuda-11080, cuda-11070]");

    #[cfg(feature = "cuda-version-from-build-system")]
    cuda_version_from_build_system();

    #[cfg(feature = "dynamic-linking")]
    dynamic_linking();
}

#[allow(unused)]
fn cuda_version_from_build_system() {
    let toolkit_root = root_candidates()
            .find(|path| path.join("include").join("cuda.h").is_file())
            .unwrap_or_else(|| {
                panic!(
                    "Unable to find `include/cuda.h` under any of: {:?}. Set the `CUDA_ROOT` environment variable to `$CUDA_ROOT/include/cuda.h` to override path.",
                    root_candidates().collect::<Vec<_>>()
                )
            });

    use std::{fs::File, io::Read};
    let mut header = File::open(toolkit_root.join("include").join("cuda.h")).unwrap();
    let mut contents = String::new();
    header.read_to_string(&mut contents).unwrap();

    let key = "CUDA_VERSION ";
    let start = key.len() + contents.find(key).unwrap();
    match contents[start..].lines().next().unwrap() {
        "12030" => println!("cargo:rustc-cfg=feature=\"cuda-12030\""),
        "12020" => println!("cargo:rustc-cfg=feature=\"cuda-12020\""),
        "12010" => println!("cargo:rustc-cfg=feature=\"cuda-12010\""),
        "12000" => println!("cargo:rustc-cfg=feature=\"cuda-12000\""),
        "11080" => println!("cargo:rustc-cfg=feature=\"cuda-11080\""),
        "11070" => println!("cargo:rustc-cfg=feature=\"cuda-11070\""),
        v => panic!("Unsupported cuda toolkit version: `{v}`. Please raise a github issue."),
    }
}

#[allow(unused)]
fn dynamic_linking() {
    let candidates: Vec<PathBuf> = root_candidates().collect();

    let toolkit_root = root_candidates()
        .find(|path| path.join("include").join("cuda.h").is_file())
        .unwrap_or_else(|| {
            panic!(
                "Unable to find `include/cuda.h` under any of: {:?}. Set the `CUDA_ROOT` environment variable to `$CUDA_ROOT/include/cuda.h` to override path.",
                candidates
            )
        });

    for path in lib_candidates(&toolkit_root) {
        println!("cargo:rustc-link-search=native={}", path.display());
    }

    #[cfg(feature = "cudnn")]
    {
        let cudnn_root = root_candidates()
            .find(|path| path.join("include").join("cudnn.h").is_file())
            .unwrap_or_else(|| {
                panic!(
                    "Unable to find `include/cudnn.h` under any of: {:?}. Set the `CUDNN_LIB` environment variable to `$CUDNN_LIB/include/cudnn.h` to override path.",
                    candidates
                )
            });

        for path in lib_candidates(&cudnn_root) {
            println!("cargo:rustc-link-search=native={}", path.display());
        }
    }

    #[cfg(feature = "driver")]
    println!("cargo:rustc-link-lib=dylib=cuda");
    #[cfg(feature = "nccl")]
    println!("cargo:rustc-link-lib=dylib=nccl");
    #[cfg(feature = "nvrtc")]
    println!("cargo:rustc-link-lib=dylib=nvrtc");
    #[cfg(feature = "curand")]
    println!("cargo:rustc-link-lib=dylib=curand");
    #[cfg(feature = "cublas")]
    println!("cargo:rustc-link-lib=dylib=cublas");
    #[cfg(any(feature = "cublas", feature = "cublaslt"))]
    println!("cargo:rustc-link-lib=dylib=cublasLt");
    #[cfg(feature = "cudnn")]
    println!("cargo:rustc-link-lib=dylib=cudnn");
}

#[allow(unused)]
fn root_candidates() -> impl Iterator<Item = PathBuf> {
    let env_vars = [
        "CUDA_PATH",
        "CUDA_ROOT",
        "CUDA_TOOLKIT_ROOT_DIR",
        "CUDNN_LIB",
    ];
    let env_vars = env_vars
        .into_iter()
        .map(std::env::var)
        .filter_map(Result::ok);

    let roots = [
        "/usr",
        "/usr/local/cuda",
        "/opt/cuda",
        "/usr/lib/cuda",
        "C:/Program Files/NVIDIA GPU Computing Toolkit",
        "C:/CUDA",
    ];
    let roots = roots.into_iter().map(Into::into);
    env_vars.chain(roots).map(Into::<PathBuf>::into)
}

#[allow(unused)]
fn lib_candidates(root: &Path) -> Vec<PathBuf> {
    [
        "lib",
        "lib/x64",
        "lib/Win32",
        "lib/x86_64",
        "lib/x86_64-linux-gnu",
        "lib64",
        "lib64/stubs",
        "targets/x86_64-linux",
        "targets/x86_64-linux/lib",
        "targets/x86_64-linux/lib/stubs",
    ]
    .iter()
    .map(|&p| root.join(p))
    .filter(|p| p.is_dir())
    .collect()
}
