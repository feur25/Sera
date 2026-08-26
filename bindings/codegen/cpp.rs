pub(super) fn gen_cpp_header() -> String {
    let mut s = String::new();
    s.push_str("#pragma once\n#include <string>\nextern \"C\" {\n");
    s.push_str("    void seraplot_free(char* ptr);\n");
    s.push_str(CPP_INTROSPECTION_EXTERN_DECLS);
    s.push_str("}\nnamespace seraplot {\n");
    s.push_str("    inline void free_str(char* p) { seraplot_free(p); }\n");
    s.push_str(CPP_INTROSPECTION_WRAPPERS);
    s.push_str("}\n");
    s
}

const CPP_INTROSPECTION_EXTERN_DECLS: &str = "    char* sera_call(const char* name, const char* json);
    char* sera_list();
    char* sera_version();
    char* sera_params_json(const char* chart, const char* variant);
    char* sera_required_params_json(const char* chart, const char* variant);
    char* sera_chart_variants_json();
    char* sera_chart_themes_json();
    char* sera_scenes3d_json();
    char* sera_docs_json();
    char* sera_themes_list();
    void sera_set_theme(const char* name);
    void sera_set_bg(const char* color);
    void sera_reset_bg();
    char* sera_demos_list();
    char* sera_demo_code(const char* name, const char* variant);
    char* sera_chart_aliases_json();
    bool sera_alias_add(const char* method, const char* alias);
    bool sera_alias_remove(const char* method, const char* alias);
    void sera_alias_reset();
    char* sera_alias_list();
    char* sera_alias_resolve(const char* name);
    char* sera_alias_save(const char* path);
    bool sera_alias_load(const char* path);
    bool sera_alias_load_json(const char* json);
";

const CPP_INTROSPECTION_WRAPPERS: &str = "    inline std::string call_by_name(const std::string& name, const std::string& json) {
        char* r = sera_call(name.c_str(), json.c_str());
        std::string out(r); free_str(r); return out;
    }
    inline std::string list() {
        char* r = sera_list();
        std::string out(r); free_str(r); return out;
    }
    inline std::string version() {
        char* r = sera_version();
        std::string out(r); free_str(r); return out;
    }
    inline std::string params(const char* chart = nullptr, const char* variant = nullptr) {
        char* r = sera_params_json(chart, variant);
        std::string out(r); free_str(r); return out;
    }
    inline std::string requiredParams(const char* chart = nullptr, const char* variant = nullptr) {
        char* r = sera_required_params_json(chart, variant);
        std::string out(r); free_str(r); return out;
    }
    inline std::string chartVariants() {
        char* r = sera_chart_variants_json();
        std::string out(r); free_str(r); return out;
    }
    inline std::string chartThemes() {
        char* r = sera_chart_themes_json();
        std::string out(r); free_str(r); return out;
    }
    inline std::string scenes3d() {
        char* r = sera_scenes3d_json();
        std::string out(r); free_str(r); return out;
    }
    inline std::string docs() {
        char* r = sera_docs_json();
        std::string out(r); free_str(r); return out;
    }
    inline std::string themes() {
        char* r = sera_themes_list();
        std::string out(r); free_str(r); return out;
    }
    inline void setTheme(const std::string& name) { sera_set_theme(name.c_str()); }
    inline void setBackground(const std::string& color) { sera_set_bg(color.c_str()); }
    inline void resetBackground() { sera_reset_bg(); }
    inline std::string demos() {
        char* r = sera_demos_list();
        std::string out(r); free_str(r); return out;
    }
    inline std::string demoCode(const std::string& name, const char* variant = nullptr) {
        char* r = sera_demo_code(name.c_str(), variant);
        std::string out(r); free_str(r); return out;
    }
    inline std::string chartAliases() {
        char* r = sera_chart_aliases_json();
        std::string out(r); free_str(r); return out;
    }
    inline bool aliasAdd(const std::string& method, const std::string& alias) {
        return sera_alias_add(method.c_str(), alias.c_str());
    }
    inline bool aliasRemove(const std::string& method, const std::string& alias) {
        return sera_alias_remove(method.c_str(), alias.c_str());
    }
    inline void aliasReset() { sera_alias_reset(); }
    inline std::string aliasList() {
        char* r = sera_alias_list();
        std::string out(r); free_str(r); return out;
    }
    inline std::string aliasResolve(const std::string& name) {
        char* r = sera_alias_resolve(name.c_str());
        std::string out(r); free_str(r); return out;
    }
    inline std::string aliasSave(const char* path = nullptr) {
        char* r = sera_alias_save(path);
        std::string out(r ? r : \"\"); free_str(r); return out;
    }
    inline bool aliasLoad(const char* path = nullptr) { return sera_alias_load(path); }
    inline bool aliasLoadJson(const std::string& json) { return sera_alias_load_json(json.c_str()); }
";
