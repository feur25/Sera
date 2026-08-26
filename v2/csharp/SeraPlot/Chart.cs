namespace SeraPlot;

public sealed class Chart
{
    public string Html { get; private set; }

    public Chart(string html) => Html = html;

    public void Save(string path) => File.WriteAllText(path, Html);

    public override string ToString() => Html;

    public static implicit operator string(Chart chart) => chart.Html;

    public Chart Call(string name, string argsJson = "{}")
    {
        var resolved = Api.ChartMethodNames.TryGetValue(name, out var canonical) ? canonical : name;
        var handle = Api.ChartFromHtml(Html);
        var updated = Api.ChartCall(handle, resolved, argsJson);
        Api.ChartFree(handle);
        if (updated == IntPtr.Zero) throw new InvalidOperationException($"Methode chart inconnue : {name}");
        Html = Api.ChartHtml(updated);
        Api.ChartFree(updated);
        return this;
    }

    public string ToSvg()
    {
        var handle = Api.ChartFromHtml(Html);
        var svg = Api.ChartToSvg(handle);
        Api.ChartFree(handle);
        return svg;
    }
}
