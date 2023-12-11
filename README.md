# ghost_suckers

graveyard font: https://www.1001fonts.com/graveyard-brk-font.html
Low Poly Graveyard: https://sketchfab.com/3d-models/low-poly-graveyard-bfa0014419ec4295addfa46bd7e21c7b
frost overlay: <a href="https://www.freepik.com/free-photo/ice-surface-texture-macro-shot-blue-wallpaper_11435892.htm#query=frozen%20overlay&position=3&from_view=keyword&track=ais&uuid=54d1f929-4130-4536-8497-a762ecaffee2">Image by rawpixel.com</a> on Freepik
wave end sound: https://freesound.org/people/SergeQuadrado/sounds/567204/
upgrade sound: https://freesound.org/people/CrazyFrog249/sounds/161628/
suck pop sound: https://freesound.org/people/joedeshon/sounds/81150/

# collisions
|        |        |        | Membership |        |      |         |
|--------|--------|--------|------------|--------|------|---------|
|        |        | Player | Ghost      | Vacuum | Wall | Upgrade |
|        | Player |    ❌   |      ✅     |    ❌   |   ✅  |   ❌  |
| Filter | Ghost  |    ✅   |      ❌     |    ✅   |   ❌  |   ❌  |
|        | Vacuum |    ❌   |      ✅     |    ❌   |   ❌  |   ✅  |
|        | Wall   |    ✅   |      ❌     |    ❌   |   ❌  |   ❌  |

# blender workflow
For every model there is a .blend file which is the original file.
It will be saved as _export.blend where all modifiers will be applied.
From there it will be exported into .glb assets.

# upgrades
more range depth
more range width
less slowing from damage

# todo
fix sound https://github.com/rparrett/bevy_pipelines_ready

# bugs
vacuum sound stays after restart (fixed?)


# nice to have
spawn ghosts out of graves with a little intro animation
map-generation
reload time
rotational camera shake
normalize distance of sucked ghosts

# experiments
## code
```Rust

/// Creates the vertices and the indices of a prism that spans the whole up/down axis
/// and widens moving out from the origin
fn create_vacuum_range() -> (Vec<Vect>, Vec<[u32; 3]>) {
    let vertices = vec![
        // right hand corners
        Vect::new(0.5, 0.5, 1.0),
        Vect::new(1.5, -1.0, 1.0),
        Vect::new(1.5, -1.0, -1.0),
        Vect::new(0.5, 0.5, -1.0),
        // left hand corners
        Vect::new(-0.5, 0.5, 1.0),
        Vect::new(-1.5, -1.0, 1.0),
        Vect::new(-1.5, -1.0, -1.0),
        Vect::new(-0.5, 0.5, -1.0),
        ];

    let indices: Vec<[u32; 3]> = vec![
        // right hand wall
        [0, 1, 2],
        [0, 2, 3],
        // left hand wall
        [4, 5, 6],
        [4, 6, 7],
        //front wall
        [1, 5, 6],
        [1, 6, 2],
        // back wall
        [0, 4, 7],
        [0, 7, 3],
        // top wall,
        [3, 2, 6],
        [3, 6, 7],
        // bottom wall,
        [0, 1, 5],
        [0, 5, 4],
        ];

    (vertices, indices)
}
```

This code creates a prettier collision shape for the vacuum, but has some issues I didn't have time to debug.