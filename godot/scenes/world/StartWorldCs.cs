using Godot;
using System;

public partial class StartWorldCs : Node
{
    [Export]
    public PackedScene BoxScene { get; set; }

    private Timer _spawnTimer;
    private Label _uiLabel;

    private int _spawnedCount = 0;

    public override void _Ready()
    {
        _spawnTimer = GetNode<Timer>("SpawnTimer");
        _uiLabel = GetNode<Label>("CanvasLayer/Label");

        _spawnedCount = 0;
    }

    // ถ้าจะใช้ input แบบเดิม
    // public override void _UnhandledInput(InputEvent @event)
    // {
    //     if (@event.IsActionPressed("ui_accept"))
    //     {
    //         var box3d = BoxScene.Instantiate<Node3D>();
    //         box3d.Position = new Vector3(
    //             GD.RandRange(-5, 5),
    //             GD.RandRange(3, 5),
    //             GD.RandRange(-5, 5)
    //         );
    //         AddChild(box3d);
    //     }
    // }

    private void SpawnBox()
    {
        int width = 10;
        int height = 8;
        float spacing = 1.0f;

        for (int y = 0; y < height; y++)
        {
            for (int x = -5; x < width; x++)
            {
                var box3d = BoxScene.Instantiate<Node3D>();

                _spawnedCount += 80; // ⚠️ เหมือน Rust (อาจจะไม่ตรงจริง)

                var pos = new Vector3(
                    x * spacing,
                    y * spacing,
                    0.0f
                );

                box3d.Position = pos;

                AddChild(box3d);
            }
        }

        GD.Print($"Spawned {_spawnedCount} boxes!");

        double fps = Engine.GetFramesPerSecond();
        string text = $"C#\nFPS: {fps}\nSpawn Count: {_spawnedCount}";
        _uiLabel.Text = text;
    }

    // ต้อง bind กับ signal จาก Timer ใน editor
    private void OnSpawnTimerTimeout()
    {
        SpawnBox();
    }
}