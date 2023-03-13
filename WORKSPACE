load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "aspect_rules_webpack",
    sha256 = "4f30fb310d625a4045e37b9e04afb2366c56b547a73c935f308e3d9c31b77519",
    strip_prefix = "rules_webpack-0.9.1",
    url = "https://github.com/aspect-build/rules_webpack/releases/download/v0.9.1/rules_webpack-v0.9.1.tar.gz",
)

#######################
# rules_webpack setup #
#######################

# Fetch the Bazel module dependencies

load("@aspect_rules_webpack//webpack:dependencies.bzl", "rules_webpack_dependencies")

rules_webpack_dependencies()

# Fetch and register the webpack tool
load("@aspect_rules_webpack//webpack:repositories.bzl", "webpack_repositories")

webpack_repositories(name = "webpack")

load("@webpack//:npm_repositories.bzl", webpack_npm_repositories = "npm_repositories")

webpack_npm_repositories()

http_archive(
    name = "rules_rust",
    sha256 = "2466e5b2514772e84f9009010797b9cd4b51c1e6445bbd5b5e24848d90e6fb2e",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.18.0/rules_rust-v0.18.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains()

load("@rules_rust//wasm_bindgen:repositories.bzl", "rust_wasm_bindgen_repositories")

rust_wasm_bindgen_repositories()

http_archive(
    name = "aspect_rules_ts",
    sha256 = "58b6c0ad158fc42883dafa157f1a25cddd65bcd788a772620192ac9ceefa0d78",
    strip_prefix = "rules_ts-1.3.2",
    url = "https://github.com/aspect-build/rules_ts/releases/download/v1.3.2/rules_ts-v1.3.2.tar.gz",
)

##################
# rules_ts setup #
##################
# Fetches the rules_ts dependencies.
# If you want to have a different version of some dependency,
# you should fetch it *before* calling this.
# Alternatively, you can skip calling this function, so long as you've
# already fetched all the dependencies.
load("@aspect_rules_ts//ts:repositories.bzl", "rules_ts_dependencies")

rules_ts_dependencies(
    # This keeps the TypeScript version in-sync with the editor, which is typically best.
    ts_version_from = "//:package.json",
)

# Fetch and register node, if you haven't already
load("@rules_nodejs//nodejs:repositories.bzl", "DEFAULT_NODE_VERSION", "nodejs_register_toolchains")

nodejs_register_toolchains(
    name = "node",
    node_version = DEFAULT_NODE_VERSION,
)

# Register aspect_bazel_lib toolchains;
# If you use npm_translate_lock or npm_import from aspect_rules_js you can omit this block.
load("@aspect_bazel_lib//lib:repositories.bzl", "register_copy_directory_toolchains", "register_copy_to_directory_toolchains")

register_copy_directory_toolchains()

register_copy_to_directory_toolchains()

###########################################
# A pnpm workspace so we can test 3p deps
load("@aspect_rules_js//npm:npm_import.bzl", "npm_translate_lock")

npm_translate_lock(
    name = "npm_deps",
    #    package_json = "//:package.json",
    pnpm_lock = "//:pnpm-lock.yaml",
    #    update_pnpm_lock = True,
    verify_node_modules_ignored = "//:.bazelignore",
    #    yarn_lock = "//:yarn.lock",
)

load("@npm_deps//:repositories.bzl", "npm_repositories")

npm_repositories()

load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository")

crates_repository(
    name = "crate_index",
    annotations = {
        "clap": [crate.annotation(
            crate_features = [
                "std",
                "help",
                "usage",
                "error-context",
                "suggestions",
            ],
        )],
        "rustyline": [crate.annotation(
            crate_features = [
                # "custom-bindings",
                # "with-dirs",
            ],
        )],
    },
    cargo_lockfile = "//:Cargo.lock",
    lockfile = "//:Cargo.Bazel.lock",
    manifests = [
        "//allocative/allocative:Cargo.toml",
        "//allocative/allocative_derive:Cargo.toml",
        "//gazebo/dupe:Cargo.toml",
        "//gazebo/gazebo:Cargo.toml",
        "//gazebo/gazebo_derive:Cargo.toml",
        "//starlark_derive:Cargo.toml",
        "//starlark:Cargo.toml",
        "//:Cargo.toml",
    ],
    packages = {
        "lalrpop": crate.spec(
            version = "0.19.7",
        ),
        "rustix": crate.spec(
            version = "0.36.9",
            default_features = False,
            features = [
            ],
        ),
    },
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()

http_archive(
    name = "io_bazel_stardoc",
    sha256 = "3fd8fec4ddec3c670bd810904e2e33170bedfe12f90adf943508184be458c8bb",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/stardoc/releases/download/0.5.3/stardoc-0.5.3.tar.gz",
        "https://github.com/bazelbuild/stardoc/releases/download/0.5.3/stardoc-0.5.3.tar.gz",
    ],
)

load("@io_bazel_stardoc//:setup.bzl", "stardoc_repositories")

stardoc_repositories()
