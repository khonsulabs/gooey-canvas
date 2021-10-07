use devx_cmd::run;
use fs_extra::dir::CopyOptions;
use khonsu_tools::{anyhow, code_coverage::CodeCoverage};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Args {
    BuildBrowserExample {
        name: Option<String>,
    },
    GenerateCodeCoverageReport {
        #[structopt(long = "install-dependencies")]
        install_dependencies: bool,
    },
    GenerateExampleSnapshots,
}

fn main() -> anyhow::Result<()> {
    let args = Args::from_args();
    match args {
        Args::BuildBrowserExample { name } => {
            build_browser_example(name.unwrap_or_else(|| String::from("basic")))?
        }
        Args::GenerateCodeCoverageReport {
            install_dependencies,
        } => CodeCoverage::<CodeCoverageConfig>::execute(install_dependencies)?,
        Args::GenerateExampleSnapshots => generate_example_snapshots()?,
    };
    Ok(())
}

fn build_browser_example(name: String) -> Result<(), anyhow::Error> {
    build_regular_browser_example(&name)?;
    execute_wasm_bindgen(
        &format!("target/wasm32-unknown-unknown/debug/examples/{}.wasm", name),
        "gooey-canvas/examples/browser/pkg/",
    )?;

    fs_extra::copy_items(
        &["gooey-canvas/assets"],
        &"gooey-canvas/examples/browser/assets",
        &CopyOptions {
            skip_exist: true,
            copy_inside: true,
            ..CopyOptions::default()
        },
    )?;

    let index_path = format!("index.html?{}", name);
    let browser_path = "gooey-canvas/examples/browser/".to_owned();

    println!(
        "Build succeeded. .{}/{} can be loaded through any http server that supports wasm.",
        browser_path, index_path,
    );
    println!();
    println!("For example, using `miniserve` (`cargo install miniserve`):");
    println!();
    println!("miniserve {}", browser_path);
    println!();
    println!("Then, navigate to: http://localhost:8080/{}", index_path);

    Ok(())
}

fn build_regular_browser_example(name: &str) -> Result<(), devx_cmd::Error> {
    println!("Executing cargo build");
    run!(
        "cargo",
        "build",
        "--example",
        name,
        "--no-default-features",
        "--features",
        "frontend-browser",
        "--target",
        "wasm32-unknown-unknown",
        "--target-dir",
        "target/wasm",
    )
}

fn execute_wasm_bindgen(wasm_path: &str, out_path: &str) -> Result<(), devx_cmd::Error> {
    println!("Executing wasm-bindgen (cargo install wasm-bindgen if you don't have this)");
    run!(
        "wasm-bindgen",
        wasm_path,
        "--target",
        "web",
        "--out-dir",
        out_path,
        "--remove-producers-section"
    )
}

struct CodeCoverageConfig;

impl khonsu_tools::code_coverage::Config for CodeCoverageConfig {
    fn ignore_paths() -> Vec<String> {
        vec![String::from("gooey-canvas/examples/*")]
    }
}

fn generate_example_snapshots() -> Result<(), devx_cmd::Error> {
    println!("Executing wasm-bindgen (cargo install wasm-bindgen if you don't have this)");
    run!("cargo", "test", "--examples", "--all-features")?;
    run!("cp", "-r", "target/snapshots", "gooey-canvas/examples/")
}
