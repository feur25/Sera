use std::fs;
use std::path::Path;

pub(crate) fn write_demo_registry(out_dir: &Path, demo_entries: &[(String, String, String)]) {
    let mut body = String::from("pub static DEMO_REGISTRY: &[(&str, &str, &str)] = &[\n");
    for (f, v, k) in demo_entries {
        body.push_str("    (\"");
        body.push_str(f);
        body.push_str("\", \"");
        body.push_str(v);
        body.push_str("\", \"");
        body.push_str(k);
        body.push_str("\"),\n");
    }
    body.push_str("];\n");
    fs::write(out_dir.join("demo_registry.rs"), body).expect("write demo_registry.rs");
}

pub(crate) fn write_params_registry(
    out_dir: &Path,
    param_entries: &[(String, String, Vec<String>)],
) {
    let mut pbody = String::from("pub static PARAMS_REGISTRY: &[(&str, &str, &[&str])] = &[\n");
    for (f, v, ps) in param_entries {
        pbody.push_str("    (\"");
        pbody.push_str(f);
        pbody.push_str("\", \"");
        pbody.push_str(v);
        pbody.push_str("\", &[");
        for (i, p) in ps.iter().enumerate() {
            if i > 0 {
                pbody.push_str(", ");
            }
            pbody.push('"');
            pbody.push_str(p);
            pbody.push('"');
        }
        pbody.push_str("]),\n");
    }
    pbody.push_str("];\n");
    fs::write(out_dir.join("params_registry.rs"), pbody).expect("write params_registry.rs");
}

pub(crate) fn write_required_registry(
    out_dir: &Path,
    required_entries: &[(String, String, Vec<String>)],
) {
    let mut rbody = String::from("pub static REQUIRED_REGISTRY: &[(&str, &str, &[&str])] = &[\n");
    for (f, v, ps) in required_entries {
        rbody.push_str("    (\"");
        rbody.push_str(f);
        rbody.push_str("\", \"");
        rbody.push_str(v);
        rbody.push_str("\", &[");
        for (i, p) in ps.iter().enumerate() {
            if i > 0 {
                rbody.push_str(", ");
            }
            rbody.push('"');
            rbody.push_str(p);
            rbody.push('"');
        }
        rbody.push_str("]),\n");
    }
    rbody.push_str("];\n");
    fs::write(out_dir.join("required_registry.rs"), rbody).expect("write required_registry.rs");
}

pub(crate) fn write_sera_aliases(out_dir: &Path, alias_entries: &[(String, Vec<String>)]) {
    let mut abody = String::from("pub static SERA_ALIASES: &[(&str, &[&str])] = &[\n");
    for (k, al) in alias_entries {
        abody.push_str("    (\"");
        abody.push_str(k);
        abody.push_str("\", &[");
        for (i, a) in al.iter().enumerate() {
            if i > 0 {
                abody.push_str(", ");
            }
            abody.push('"');
            abody.push_str(a);
            abody.push('"');
        }
        abody.push_str("]),\n");
    }
    abody.push_str("];\n");
    fs::write(out_dir.join("sera_aliases.rs"), abody).expect("write sera_aliases.rs");
}

pub(crate) fn write_chart_alias_registry(out_dir: &Path, chart_alias_pairs: &[(String, String)]) {
    let mut cbody = String::from("pub static CHART_ALIAS_REGISTRY: &[(&str, &str)] = &[\n");
    for (a, fname) in chart_alias_pairs {
        cbody.push_str("    (\"");
        cbody.push_str(a);
        cbody.push_str("\", \"");
        cbody.push_str(fname);
        cbody.push_str("\"),\n");
    }
    cbody.push_str("];\n");
    fs::write(out_dir.join("chart_alias_registry.rs"), cbody)
        .expect("write chart_alias_registry.rs");
}
