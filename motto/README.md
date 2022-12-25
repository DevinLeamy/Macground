# Macground


### Configuration


### Background
Sources:
- User defined
    - `String` (url)
- Random image
    - `String` (api key)
- Background color
    - `String`

### Text
Sources:
- User input
    - `String`
- Random word
- Random quote

### Font
- Font size
    - (Optional) `u32`
- Font color
    - `String`
- Font (ttf/otf)
    - `String`

```rust
#[derive(Serialize, Deserialize, Default)]
enum BackgroundOptions {
    Url(String),
    RandomImage,
    Color(String), // "random" / "red", "green", "teal" / "rgb(20, 48, 200)" / "#FE7789"
}

#[derive(Serialize, Deserialize, Default)]
enum TextOptions {
    Message(String),
    RandomQuote(String), // Requires an API key (currently)
    RandomWord,
}

#[derive(Serialize, Deserialize, Default)]
struct FontOptions {
    font: String,
    color: Rgba<u8>,
    font_size: u32, // Represents a PxScale
}

// We want this to be serializable/deserializable s.t. users can
// save layouts.
#[derive(Serialize, Deserialize, Default)]
struct Options {
    background: BackgroundOptions,
    text: TextOptions,
    font: FontOptions,
}
```

### TODO
- [ ] Argument parser 
- [ ] Text boxes -> Layouts
- [ ] Font loader being shit
- [ ] Font size adjust to "reasonably" fit the screen
- [ ] API for creating and using new sources 
- [ ] Add option to save to a file location
- [ ] Make the text size "fit" the text box
