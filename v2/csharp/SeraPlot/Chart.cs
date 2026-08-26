namespace SeraPlot;

public readonly struct Chart
{
    public string Html { get; }

    public Chart(string html) => Html = html;

    public void Save(string path) => File.WriteAllText(path, Html);

    public override string ToString() => Html;

    public static implicit operator string(Chart chart) => chart.Html;
}
