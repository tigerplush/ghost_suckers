# ghost_suckers

graveyard font: https://www.1001fonts.com/graveyard-brk-font.html
Low Poly Graveyard: https://sketchfab.com/3d-models/low-poly-graveyard-bfa0014419ec4295addfa46bd7e21c7b
frost overlay: <a href="https://www.freepik.com/free-photo/ice-surface-texture-macro-shot-blue-wallpaper_11435892.htm#query=frozen%20overlay&position=3&from_view=keyword&track=ais&uuid=54d1f929-4130-4536-8497-a762ecaffee2">Image by rawpixel.com</a> on Freepik

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
sound
enemy wave spawner

# nice to have
map-generation
reload time
camera shake
normalize distance of sucked ghosts