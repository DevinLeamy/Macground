from PIL import Image, ImageDraw, ImageFont, ImageColor
import sys
import random
import os


(WIDTH, HEIGHT) = (3840, 2160)
FONTS = [
    "font1.otf",
    "font2.ttf",
    "font3.ttf",
    "font4.ttf",
    "font5.otf",
    "font6.otf",
    "font7.ttf"
]
OUTPUT_PATH = "/Users/Devin/Desktop/Github/DevinLeamy/Macground/images"
FONT_PATH = "/Users/Devin/Desktop/Github/DevinLeamy/Macground/fonts"

def get_wrapped_text(text: str, font: ImageFont.ImageFont, line_length: int):
    lines = ['']
    for word in text.split():
        line = f'{lines[-1]} {word}'.strip()
        if font.getlength(line) <= line_length:
            lines[-1] = line
        else:
            lines.append(word)
    return '\n'.join(lines)

"""
    Draws {message} centered within {image} with the given {font}
    and {text_color}
"""
def draw_centered_text(image: Image.Image, message: str, font: ImageFont.ImageFont, text_color: str="white", center_offset_y: int=0):
    image_draw = ImageDraw.Draw(image)

    formatted_message = get_wrapped_text(message, font, line_length=3000)

    text_width, text_height = image_draw.multiline_textsize(formatted_message, font=font)

    x_text = (WIDTH - text_width) / 2
    y_text = (HEIGHT - text_height) / 2 + center_offset_y

    image_draw.multiline_text((x_text, y_text), formatted_message, font=font, fill=text_color)

def random_color():
    r = random.randint(0, 255)
    g = random.randint(0, 255)
    b = random.randint(0, 255)

    return f"rgb({r}, {g}, {b})"

def create_background_image(message):
    font_file = f"{FONT_PATH}/{random.choice(FONTS)}"
    font = ImageFont.truetype(font_file, size=300)
    background_color = ImageColor.getrgb(random_color())
    file_name = f"background_image_{random.randint(1000, 9999)}.png"
    output_image_path = f"{OUTPUT_PATH}/{file_name}"

    image = Image.new("RGB", (WIDTH, HEIGHT), color=background_color)

    draw_centered_text(image, message, font, center_offset_y=-100)

    image.save(output_image_path)

    return output_image_path

def set_background_image(image_path):
    try:
        os.system(f"osascript -e \'tell application \"Finder\" to set desktop picture to POSIX file \"{image_path}\"\'")
    except:
        print("Error: failed to set background image")

if __name__ == "__main__":
    message = sys.argv[1]
    background_image_path = create_background_image(message)
    set_background_image(background_image_path)


