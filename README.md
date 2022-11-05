## Presented by `Fe`tching
*(better rust/iron pun TBA)*
- George Huebner (georgeh3)

# Project Rationale

I've wanted to make a game for a while now, and although the Rust gamedev ecosystem is still far from the maturity of Unity or Unreal, I like Rust infinitely more than C# and even C++. I'm still not 100% if I want to use Bevy or Macroquad, but I think the latter would be a little easier to tinker around with. Trying to implement stuff like audio/rendering is out of the scope of this project, but I do think it's cool that Macroquad can target WASM, because more people will be willing to try your game in the browser.

I also considered fleshing out a `ls-R` to tectonic mapping system, but I'm no Donald Knuth --- I don't hate myself *that* much. This is my backup of something that is more broadly useful if for some reason things go really far south and I have to abandon ship on the game idea.

For theme/game mechanics, I wanted to explore a concept mashup of a classic dungeon crawler like Zelda with the tactical turn based combat of Fire Emblem. Choosing a 2D game offloads a ton of asset work and sandbox design --- as a fair warning, I am planning on copying most, if not all, of my assets from other games (with attribution, of course). I worked on a Fire Emblem clone in Python once before, and most stuff boiled down to stat calculations and turn management, so I feel decently confident that a working prototype is feasible.

![](https://assets.reedpopcdn.com/eurogamer-yau868.jpg/BROK/resize/1200x1200%3E/format/jpg/quality/70/eurogamer-yau868.jpg)
![](https://www.fireemblemwod.com/fe6/mapasfe6/14.png)

Restricting players to specific movesets can create new challenges, but not with the same sandbox of Zelda (i.e. chucking bombs everywhere)
