fn pascal_case(s: &str) -> String {
    s.split('_')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().to_string() + c.as_str(),
            }
        })
        .collect()
}

pub(super) fn gen_csharp() -> String {
    let mut s = String::new();
    s.push_str("#nullable enable\n");
    s.push_str("using System;\n");
    s.push_str("using System.Collections.Generic;\n");
    s.push_str("using System.Runtime.InteropServices;\n");
    s.push_str("namespace SeraPlot {\n");
    s.push_str("    public static partial class Api {\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"seraplot_free\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        private static extern void Free(IntPtr ptr);\n\n");
    s.push_str("        private static string Call(IntPtr ptr) {\n");
    s.push_str("            var r = Marshal.PtrToStringUTF8(ptr) ?? string.Empty;\n");
    s.push_str("            Free(ptr); return r;\n");
    s.push_str("        }\n\n");
    s.push_str(&gen_csharp_introspection());
    s.push_str("    }\n}\n");
    s
}

fn gen_csharp_introspection() -> String {
    let mut s = String::new();
    s.push_str("\n        [DllImport(\"seraplot\", EntryPoint = \"sera_call\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        private static extern IntPtr _call_by_name([MarshalAs(UnmanagedType.LPUTF8Str)] string name, [MarshalAs(UnmanagedType.LPUTF8Str)] string json);\n");
    s.push_str("        public static Chart CallByName(string name, string json) => new Chart(Call(_call_by_name(name, json)));\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_list\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        private static extern IntPtr _list();\n");
    s.push_str("        public static string List() => Call(_list());\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_version\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        private static extern IntPtr _version();\n");
    s.push_str("        public static string Version() => Call(_version());\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_params_json\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        private static extern IntPtr _params_json([MarshalAs(UnmanagedType.LPUTF8Str)] string? chart, [MarshalAs(UnmanagedType.LPUTF8Str)] string? variant);\n");
    s.push_str("        public static string Params(string? chart = null, string? variant = null) => Call(_params_json(chart, variant));\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_required_params_json\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        private static extern IntPtr _required_params_json([MarshalAs(UnmanagedType.LPUTF8Str)] string? chart, [MarshalAs(UnmanagedType.LPUTF8Str)] string? variant);\n");
    s.push_str("        public static string RequiredParams(string? chart = null, string? variant = null) => Call(_required_params_json(chart, variant));\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_chart_variants_json\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        private static extern IntPtr _chart_variants_json();\n");
    s.push_str("        public static string ChartVariants() => Call(_chart_variants_json());\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_chart_themes_json\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        private static extern IntPtr _chart_themes_json();\n");
    s.push_str("        public static string ChartThemes() => Call(_chart_themes_json());\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_scenes3d_json\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        private static extern IntPtr _scenes3d_json();\n");
    s.push_str("        public static string Scenes3d() => Call(_scenes3d_json());\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_docs_json\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        private static extern IntPtr _docs_json();\n");
    s.push_str("        public static string Docs() => Call(_docs_json());\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_themes_list\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        private static extern IntPtr _themes_list();\n");
    s.push_str("        public static string Themes() => Call(_themes_list());\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_set_theme\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        public static extern void SetTheme([MarshalAs(UnmanagedType.LPUTF8Str)] string name);\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_set_bg\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        public static extern void SetBackground([MarshalAs(UnmanagedType.LPUTF8Str)] string color);\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_reset_bg\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        public static extern void ResetBackground();\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_demos_list\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        private static extern IntPtr _demos_list();\n");
    s.push_str("        public static string Demos() => Call(_demos_list());\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_demo_code\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        private static extern IntPtr _demo_code([MarshalAs(UnmanagedType.LPUTF8Str)] string name, [MarshalAs(UnmanagedType.LPUTF8Str)] string? variant);\n");
    s.push_str("        public static string DemoCode(string name, string? variant = null) => Call(_demo_code(name, variant));\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_chart_aliases_json\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        private static extern IntPtr _chart_aliases_json();\n");
    s.push_str("        public static string ChartAliases() => Call(_chart_aliases_json());\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_alias_add\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        [return: MarshalAs(UnmanagedType.U1)]\n");
    s.push_str("        public static extern bool AliasAdd([MarshalAs(UnmanagedType.LPUTF8Str)] string method, [MarshalAs(UnmanagedType.LPUTF8Str)] string alias);\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_alias_remove\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        [return: MarshalAs(UnmanagedType.U1)]\n");
    s.push_str("        public static extern bool AliasRemove([MarshalAs(UnmanagedType.LPUTF8Str)] string method, [MarshalAs(UnmanagedType.LPUTF8Str)] string alias);\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_alias_reset\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        public static extern void AliasReset();\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_alias_list\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        private static extern IntPtr _alias_list();\n");
    s.push_str("        public static string AliasList() => Call(_alias_list());\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_alias_resolve\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        private static extern IntPtr _alias_resolve([MarshalAs(UnmanagedType.LPUTF8Str)] string name);\n");
    s.push_str("        public static string AliasResolve(string name) => Call(_alias_resolve(name));\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_alias_save\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        private static extern IntPtr _alias_save([MarshalAs(UnmanagedType.LPUTF8Str)] string? path);\n");
    s.push_str("        public static string AliasSave(string? path = null) => Call(_alias_save(path));\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_alias_load\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        [return: MarshalAs(UnmanagedType.U1)]\n");
    s.push_str("        private static extern bool _alias_load([MarshalAs(UnmanagedType.LPUTF8Str)] string? path);\n");
    s.push_str("        public static bool AliasLoad(string? path = null) => _alias_load(path);\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_alias_load_json\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        [return: MarshalAs(UnmanagedType.U1)]\n");
    s.push_str("        public static extern bool AliasLoadJson([MarshalAs(UnmanagedType.LPUTF8Str)] string json);\n\n");
    s.push_str("        private static string ToPascalCase(string snake) {\n");
    s.push_str("            var sb = new System.Text.StringBuilder();\n");
    s.push_str("            foreach (var part in snake.Split('_')) {\n");
    s.push_str("                if (part.Length == 0) continue;\n");
    s.push_str("                sb.Append(char.ToUpperInvariant(part[0]));\n");
    s.push_str("                if (part.Length > 1) sb.Append(part.Substring(1));\n");
    s.push_str("            }\n");
    s.push_str("            return sb.ToString();\n");
    s.push_str("        }\n\n");
    s.push_str("        private static IReadOnlyDictionary<string, string> BuildNames() {\n");
    s.push_str("            var map = new Dictionary<string, string>();\n");
    s.push_str("            var registered = System.Text.Json.JsonSerializer.Deserialize<string[]>(List()) ?? System.Array.Empty<string>();\n");
    s.push_str("            foreach (var name in registered) map[ToPascalCase(name)] = name;\n");
    s.push_str("            var aliases = System.Text.Json.JsonSerializer.Deserialize<string[][]>(ChartAliases()) ?? System.Array.Empty<string[]>();\n");
    s.push_str("            foreach (var pair in aliases) if (pair.Length == 2) map[ToPascalCase(pair[0])] = pair[1];\n");
    s.push_str("            return map;\n");
    s.push_str("        }\n\n");
    s.push_str("        private static readonly Lazy<IReadOnlyDictionary<string, string>> _names = new(BuildNames);\n");
    s.push_str("        public static IReadOnlyDictionary<string, string> Names => _names.Value;\n\n");
    s.push_str("        public static Chart Call(string name, string json) => CallByName(Names.TryGetValue(name, out var canonical) ? canonical : name, json);\n\n");
    s.push_str(&gen_csharp_chart_handle());
    s
}

fn gen_csharp_chart_handle() -> String {
    let mut s = String::new();
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_chart_from_html\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        internal static extern IntPtr ChartFromHtml([MarshalAs(UnmanagedType.LPUTF8Str)] string html);\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_chart_html\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        private static extern IntPtr _chart_html(IntPtr chart);\n");
    s.push_str("        internal static string ChartHtml(IntPtr chart) => Call(_chart_html(chart));\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_chart_free\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        internal static extern void ChartFree(IntPtr chart);\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_chart_to_svg\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        private static extern IntPtr _chart_to_svg(IntPtr chart);\n");
    s.push_str("        internal static string ChartToSvg(IntPtr chart) {\n");
    s.push_str("            var ptr = _chart_to_svg(chart);\n");
    s.push_str("            return ptr == IntPtr.Zero ? string.Empty : Call(ptr);\n");
    s.push_str("        }\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_chart_call\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        private static extern IntPtr _chart_call(IntPtr chart, [MarshalAs(UnmanagedType.LPUTF8Str)] string method, [MarshalAs(UnmanagedType.LPUTF8Str)] string argsJson);\n");
    s.push_str("        internal static IntPtr ChartCall(IntPtr chart, string method, string argsJson) => _chart_call(chart, method, argsJson);\n\n");
    s.push_str("        [DllImport(\"seraplot\", EntryPoint = \"sera_chart_methods_json\", CallingConvention = CallingConvention.Cdecl)]\n");
    s.push_str("        private static extern IntPtr _chart_methods_json();\n");
    s.push_str("        public static string ChartMethods() => Call(_chart_methods_json());\n\n");
    s.push_str("        private static IReadOnlyDictionary<string, string> BuildChartMethodNames() {\n");
    s.push_str("            var map = new Dictionary<string, string>();\n");
    s.push_str("            var methods = System.Text.Json.JsonSerializer.Deserialize<string[]>(ChartMethods()) ?? System.Array.Empty<string>();\n");
    s.push_str("            foreach (var name in methods) map[ToPascalCase(name)] = name;\n");
    s.push_str("            return map;\n");
    s.push_str("        }\n\n");
    s.push_str("        private static readonly Lazy<IReadOnlyDictionary<string, string>> _chartMethodNames = new(BuildChartMethodNames);\n");
    s.push_str("        internal static IReadOnlyDictionary<string, string> ChartMethodNames => _chartMethodNames.Value;\n\n");
    s
}
