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

    private static string MergeArgs(object baseArgs, string extraJson)
    {
        var merged = new Dictionary<string, JsonElement>();
        using (var baseDoc = JsonDocument.Parse(JsonSerializer.Serialize(baseArgs)))
        {
            foreach (var prop in baseDoc.RootElement.EnumerateObject()) merged[prop.Name] = prop.Value.Clone();
        }
        if (!string.IsNullOrWhiteSpace(extraJson) && extraJson != "{}")
        {
            using var extraDoc = JsonDocument.Parse(extraJson);
            foreach (var prop in extraDoc.RootElement.EnumerateObject()) merged[prop.Name] = prop.Value.Clone();
        }
        return JsonSerializer.Serialize(merged);
    }

    public static Chart ExportGif(this Chart chart, string gifPath, string splsPath, string optionsJson = "{}")
    {
        Api.PulseCall(chart.Html, "export_gif", MergeArgs(new { gif_path = gifPath, spls_path = splsPath }, optionsJson));
        return chart;
    }

    public static int GifFrameCount(this Chart chart, string splsPath)
    {
        var result = Api.PulseCall(chart.Html, "gif_frame_count", JsonSerializer.Serialize(new { spls_path = splsPath }));
        using var doc = JsonDocument.Parse(result);
        return doc.RootElement.GetProperty("frame_count").GetInt32();
    }

    public static Chart GifPreviewFrame(this Chart chart, string splsPath, int frameIndex, string pngPath, string optionsJson = "{}")
    {
        Api.PulseCall(chart.Html, "gif_preview_frame", MergeArgs(new { spls_path = splsPath, frame_index = frameIndex, png_path = pngPath }, optionsJson));
        return chart;
    }

    public static Chart ExportVideo(this Chart chart, string videoPath, string splsPath, string format = "mp4", int fps = 30, string optionsJson = "{}")
    {
        Api.PulseCall(chart.Html, "export_video", MergeArgs(new { video_path = videoPath, spls_path = splsPath, format, fps }, optionsJson));
        return chart;
    }
}
