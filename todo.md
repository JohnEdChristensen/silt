
## Architecture

- [ ] State
 -  Playing
    - UI
        - Option<hovered cell>
            - should be visually highlighted
            - display summary information?
        - Option<selected cell>
            - Open window/side bar containing detailed information
                - read only data
                - read/write data
        - Camera
            - Location: Vec3
            - Rotation: North,East,Sout,West
    - World
        - Vec3D<Cell>
            - Cell
                - enum (Air,Water,Land)
                    - Option<Air | Water, Building >
                        - Trait: Building
                        - water amount
                            - fluid should occupy some space in this cell
                        - ex: bench,fence
                    - Option<Land Building > 
                        - Trait: Building 
                        - ex: road
        - Entites
            Creature
            Object
                - Location: (&Cell,RelativePos)

 -  Paused


- [ ] graphics

## Input
Mouse object collision detection
    fn (x,y) -> Option<Entity | Cell>
        - may return None if UI is selected
        - Entity | Cell | Window?
Intuitive camera controls
- wasd? scroll

## World gen
- Noise based height map gen
- assign textures based on height/biome
- Errosion

## Physics
- Simple water simulation
    - pressure?

## Camera
- [ ] locked isometric view
- [ ] discrete rotation
