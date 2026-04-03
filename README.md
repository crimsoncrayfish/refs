# refs
A pureref clone. Go support pureref (https://www.pureref.com/). Its great

# design

the system will consist of separate entities:
- world
- camera
- app
- app_state
- render functions

# world
The world will keep track of where entities exist in the world coordinates and how they are scaled/manipulated/stacked inside the world

# camera
The app will translate everything to app coordinates from the world state by accounting for offset and zoom. 

## offset and zoom
at offset (0, 0) and zoom 1 the world coordinates will overlap exactly with the app coordinates. to translate from world to app:
```rust
    fn coord_to_world_coord(&self, coord: Pos2) -> WorldPos2 {
        (coord / self.state.zoom + self.state.offset).Into()
    }
    fn world_coord_to_coord(&self, coord: WorldPos2) -> Pos2 {
        (coord).Into()+ self.state.offset / self.state.zoom 
    }
```

# app
app will connect the world and the camera with the render functions

# app_state
app state will keep track of settings and app state like "should the menu show?" etc.

# render functions
the render functions will do the job of rendering everything that falls within the camera's scope
