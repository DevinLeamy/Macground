# _Macground_

***`Macground`*** Create custom backgrounds with ease, from the command line. 
<p>
    <a href="#usage">Usage</a> •
    <a href="#options">Options</a> •
    <a href="#installation">Installation</a> •
    <a href="#platforms">Platforms</a> 
</p>


### Usage

```bash
# Displays an image with a quote set in a random color
macground --random-image --text-color "random" --random-quote

# Displays "Macground" on a maroon background with a custom font
macground --message "Macground" --color "maroon" --font "./roboto.oft"
```


### Installation
Install Macground using [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).
```bash
cargo install macground
```

### Options
```bash
Usage: macground [OPTIONS]

Options:
      --background-image <BACKGROUND_IMAGE>
          Url of a background image
      --random-image
          Flag to set the background to a random image
      --color <COLOR>
          Color of the background, if no image is set. Accepts: "<color-name>" | "rgb(...)" | "#FFAAEE" | "hsl(...)" | "random"
      --message <MESSAGE>
          Message to display to the screen
      --random-quote
          Random quote to display to the screen
      --random-word
          Random would to display to the screen
      --text-color <TEXT_COLOR>
          Color of the text, if any is displayed Accepts: "<color-name>" | "rgb(...)" | "#FFAAEE" | "hsl(...)" | "random"
      --text-size <TEXT_SIZE>
          Size of the text characters in pixels, defaults to filling the text's parent
      --font <FONT>
          Path to an otf or ttf font
  -h, --help
          Print help information
``` 

### Platforms
Macground is supported on:
- [x] MacOS 
- [ ] Windows
- [ ] Linux 

