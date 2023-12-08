# ghost_suckers

graveyard font: https://www.1001fonts.com/graveyard-brk-font.html
Low Poly Graveyard: https://sketchfab.com/3d-models/low-poly-graveyard-bfa0014419ec4295addfa46bd7e21c7b

# collisions
|        |        |        | Membership |        |      |
|--------|--------|--------|------------|--------|------|
|        |        | Player | Ghost      | Vacuum | Wall |
|        | Player |    ❌   |      ✅     |    ❌   |   ✅  |
| Filter | Ghost  |    ✅   |      ❌     |    ✅   |   ❌  |
|        | Vacuum |    ❌   |      ✅     |    ❌   |   ❌  |
|        | Wall   |    ✅   |      ❌     |    ❌   |   ❌  |

# blender workflow
For every model there is a .blend file which is the original file.
It will be saved as _export.blend where all modifiers will be applied.
From there it will be exported into .glb assets.

# upgrades
more range depth
more range width
more health
more speed
less sucking time
less slowing from damage

# todo
show vacuuming
sound
map-generation
enemy wave spawner

# nice to have
reload time
camera shake
normalize distance of sucked ghosts