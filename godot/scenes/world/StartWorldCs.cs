using Godot;
using System;

public partial class StartWorldCs : Node
{
    [Export]
    public PackedScene BoxScene;

    private Timer spawnTimer;
    private Label uiLabel;

    private int spawnedCount = 0;

    public override void _Ready()
    {
        spawnTimer = GetNode<Timer>("SpawnTimer");
        uiLabel = GetNode<Label>("CanvasLayer/Label");

        spawnedCount = 0;
        SpawnBox();
    }

    private void SpawnBox()
    {
        int spawnAmount = 5000;

        if (BoxScene == null)
        {
            GD.PushError("Box scene not assigned!");
            return;
        }

        for (int i = 0; i < spawnAmount; i++)
        {
            var boxInstance = BoxScene.Instantiate<Node3D>();

            var pos = new Vector3(
                GD.RandRange(-20, 20),
                GD.RandRange(10, 30),
                GD.RandRange(-20, 20)
            );

            boxInstance.Position = pos;

            AddChild(boxInstance);

            spawnedCount++;
        }

        // อัปเดต UI
        double fps = Engine.GetFramesPerSecond();
        string text = $"Language: C#\nFPS: {fps:F0}\nObjects: {spawnedCount}";
        uiLabel.Text = text;
    }

    private void OnSpawnTimerTimeout()
    {
        SpawnBox();
    }
}