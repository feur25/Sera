use std::env;
use std::fs;
use std::path::PathBuf;

#[path = "docgen/bindings.rs"]
mod build_bindings;
#[path = "docgen/common.rs"]
mod build_common;
#[path = "services/ml/docgen/mod.rs"]
mod build_ml;
#[path = "services/plot/docgen/mod.rs"]
mod build_plot;
#[path = "docgen/registry.rs"]
mod build_registry;

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let src_root = manifest.clone();
    let plot_root = src_root.join("services").join("plot");
    let ml_root = src_root.join("services").join("ml");
    let data_root = src_root.join("services").join("data");
    let bindings_root = src_root.join("bindings");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=services/plot");
    println!("cargo:rerun-if-changed=services/ml");
    println!("cargo:rerun-if-changed=services/data");
    println!("cargo:rerun-if-changed=bindings");
    println!("cargo:rerun-if-changed=docgen");
    println!("cargo:rerun-if-changed=services/plot/docgen");
    println!("cargo:rerun-if-changed=services/ml/docgen");

    let mut plot_files: Vec<PathBuf> = Vec::new();
    build_common::walk(&plot_root, &mut plot_files);
    plot_files.sort();

    let mut ml_files: Vec<PathBuf> = Vec::new();
    build_common::walk(&ml_root, &mut ml_files);
    ml_files.sort();

    let mut bindings_files: Vec<PathBuf> = Vec::new();
    build_common::walk(&bindings_root, &mut bindings_files);
    bindings_files.sort();

    let plot_doc_data = build_plot::docs::collect(&plot_root);
    build_plot::docs::write_registry(
        &src_root,
        &plot_root,
        &plot_doc_data.demo_entries,
        &plot_doc_data.param_entries,
    );
    build_ml::docs::write_registry(&src_root, &ml_root, &data_root);

    let mut chart_alias_pairs: Vec<(String, String)> = Vec::new();
    let mut builder_fns: Vec<String> = Vec::new();
    let mut ml_builder_fns: Vec<String> = Vec::new();
    for f in plot_files.iter().chain(ml_files.iter()).chain(bindings_files.iter()) {
        let Ok(src) = fs::read_to_string(f) else {
            continue;
        };
        if f.starts_with(&plot_root) {
            builder_fns.extend(build_common::extract_marker_fn_names(&src, "sera_builder"));
        } else if f.starts_with(&ml_root) {
            ml_builder_fns.extend(build_common::extract_marker_fn_names(&src, "sera_builder"));
        }
        for (aliases, fn_name) in build_common::extract_alias_fn_pairs(&src) {
            for alias in aliases {
                chart_alias_pairs.push((alias, fn_name.clone()));
            }
        }
    }
    chart_alias_pairs.sort();
    chart_alias_pairs.dedup();
    builder_fns.sort();
    builder_fns.dedup();
    ml_builder_fns.sort();
    ml_builder_fns.dedup();

    build_registry::write_demo_registry(&out_dir, &plot_doc_data.demo_entries);
    build_registry::write_params_registry(&out_dir, &plot_doc_data.param_entries);
    build_registry::write_sera_aliases(&out_dir, &plot_doc_data.alias_entries);
    build_registry::write_chart_alias_registry(&out_dir, &chart_alias_pairs);
    build_bindings::write_all(&src_root, &bindings_root, &out_dir, &builder_fns, &ml_builder_fns);
}
