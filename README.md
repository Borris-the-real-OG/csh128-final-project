## Presented by `Fe`tching
*(better Rust/iron pun TBA)*
- George Huebner (georgeh3)

# So Long Bevy

[![](https://img.youtube.com/vi/M1vfXoUNDYA/0.jpg)](https://www.youtube.com/watch?v=M1vfXoUNDYA)

*We've got a great name, a great team, and a GREAT name. Let's pivot!*

I have decided not to proceed with my original idea of designing a game in Bevy. There are a few reasons for this decision:

1. Bevy's ECS is cool and all, but from everything I've seen, I wouldn't really be leveraging Rust itself when making a game using an existing framework: I would be writing Bevy code, not Rust code.
    * I would really like to explore building a game from the ground up (in Rust) in the future, but this is a significantly larger endeavor than can be accomplished in a semester project.
2. The Rust gamedev ecosystem has yet to reach maturity. I, too, like to live life on the bleeding edge sometimes, but having all dependencies break each other gets very tiring very quickly.
3. I could feasibly make a prototype, but likely not one that I would actually be happy with. I knew the asset pipeline would be bad, but I underestimated that even ripping assets from existing games would still be a lot of work.

Intstead, I will be working on a compatibility layer for the Tectonic typesetting system. Why Tectonic?

1. It's open source — Other people can actually use my code, and it might even get merged upstream!
2. It solves a problem. I'm no stranger to building useless things, but having something hat I can benefit from motivates me to actually make the project good.
3. It uses Rust. Well, no duh, Bevy uses Rust too, but the whole purpose of Tectonic is to provide a safe, platform-independent wrapper over the $\LaTeX$ wrappers of yore. It makes sense to write this project in Rust: I can claim moral superiority over C# users now!

<br>

![](Untitled.png)