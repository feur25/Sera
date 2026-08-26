namespace SeraPlot;

public sealed class Chart : IDisposable
{
    private string? _html;
    private IntPtr _handle;
    private bool _disposed;

    public Chart(string html) => _html = html;

    public string Html
    {
        get
        {
            if (_handle != IntPtr.Zero) _html = Api.ChartHtml(_handle);
            return _html ?? string.Empty;
        }
    }

    public void Save(string path) => File.WriteAllText(path, Html);

    public override string ToString() => Html;

    public static implicit operator string(Chart chart) => chart.Html;

    public Chart Call(string name, string argsJson = "{}")
    {
        var resolved = Api.ChartMethodNames.TryGetValue(name, out var canonical) ? canonical : name;
        if (_handle == IntPtr.Zero) _handle = Api.ChartFromHtml(_html ?? string.Empty);
        var updated = Api.ChartCall(_handle, resolved, argsJson);
        if (updated == IntPtr.Zero) throw new InvalidOperationException($"Methode chart inconnue : {name}");
        Api.ChartFree(_handle);
        _handle = updated;
        _html = null;
        return this;
    }

    public string ToSvg()
    {
        var owned = _handle == IntPtr.Zero;
        var handle = owned ? Api.ChartFromHtml(_html ?? string.Empty) : _handle;
        var svg = Api.ChartToSvg(handle);
        if (owned) Api.ChartFree(handle);
        return svg;
    }

    public void Dispose()
    {
        if (_disposed) return;
        if (_handle != IntPtr.Zero) Api.ChartFree(_handle);
        _handle = IntPtr.Zero;
        _disposed = true;
        GC.SuppressFinalize(this);
    }

    ~Chart()
    {
        if (_handle != IntPtr.Zero) Api.ChartFree(_handle);
    }
}
