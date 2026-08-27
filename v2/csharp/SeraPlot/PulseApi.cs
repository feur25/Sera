using System.Runtime.InteropServices;
using System.Text.Json;

namespace SeraPlot;

public sealed class SeraPulseException : Exception
{
    public SeraPulseException(string message) : base(message) { }
}

public static partial class Api
{
    [DllImport("seraplot", EntryPoint = "sera_pulse_call", CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr _pulse_call(string html, string name, string json);

    public static string PulseCall(string html, string name, string json = "{}")
    {
        var raw = Call(_pulse_call(html, name, json));
        using var doc = JsonDocument.Parse(raw);
        var root = doc.RootElement;
        if (root.TryGetProperty("ok", out var okEl) && okEl.GetBoolean())
        {
            return root.TryGetProperty("result", out var resultEl) ? resultEl.GetRawText() : "{}";
        }
        var error = root.TryGetProperty("error", out var errEl) ? errEl.GetString() : null;
        throw new SeraPulseException(error ?? "seraplot pulse: unknown error");
    }
}

public static class ChartPulseExtensions
{
    public static Chart Record(this Chart chart, string path)
    {
        Api.PulseCall(chart.Html, "record", JsonSerializer.Serialize(new { path }));
        return chart;
    }

    public static Chart StopRecord(this Chart chart)
    {
        Api.PulseCall(chart.Html, "stop_record");
        return chart;
    }

    public static Chart Push(this Chart chart, int[] indices, double[] x, double[]? y = null)
    {
        var payload = y is null
            ? JsonSerializer.Serialize(new { indices, x })
            : JsonSerializer.Serialize(new { indices, x, y });
        Api.PulseCall(chart.Html, "push", payload);
        return chart;
    }

    public static Chart PushVector(this Chart chart, int[] indices, double[][] components)
    {
        Api.PulseCall(chart.Html, "push_vector", JsonSerializer.Serialize(new { indices, components }));
        return chart;
    }

    public static Chart ExportStandalone(this Chart chart, string htmlPath, string splsPath)
    {
        Api.PulseCall(chart.Html, "export_standalone", JsonSerializer.Serialize(new { html_path = htmlPath, spls_path = splsPath }));
        return chart;
    }
}
