using Godot;
using System;

public partial class MainSceneCalCs : Node
{
    private Label uiLabel;

    public override void _Ready()
    {
        uiLabel = GetNode<Label>("CanvasLayer/Label");
        RunBenchmark();
    }

    private void RunBenchmark()
    {
        int iterations = 5_000_000;

        ulong start = Time.GetTicksMsec();

        float result = 0f;

        for (int i = 0; i < iterations; i++)
        {
            float x = i;
            Vector3 v = new Vector3(Mathf.Sin(x), Mathf.Cos(x), Mathf.Tan(x));
            Vector3 n = v.Normalized();
            result += n.Length();
        }

        ulong end = Time.GetTicksMsec();
        ulong duration = end - start;

        string text = $"Language: C#\nTime: {duration} ms\nResult: {result}";
        uiLabel.Text = text;
    }
}