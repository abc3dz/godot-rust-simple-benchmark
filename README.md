My simple benchmark results (from someone not very experienced with programming terms):

I forked a benchmark project from Rayato159 and tested Godot performance using Rust, C#, and GDScript.

Spawning 5,000 objects at a time
→ C# performed the best.
Running a heavy calculation (5 million iterations per run)
→ C# also performed the best when using the Godot API.

Then I wanted to understand where Rust performs better, so I switched to pure Rust computation (without calling Godot APIs).
→ In this case, Rust performed the best.

From what I understand, this is because Rust avoids the overhead of calling Godot through GDExtension.
When Rust code frequently calls Godot APIs, it becomes slower compared to C#, since C# has tighter integration with the engine and has been optimized for years.

GDScript consistently ranked last, which makes sense since it's mainly designed for small to medium games or cases that don't require heavy computation.

This is just a simple experiment, so it may not represent all real-world scenarios.
