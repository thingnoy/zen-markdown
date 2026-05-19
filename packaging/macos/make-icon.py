#!/usr/bin/env python3
"""Generate a Tokyo Night app icon (1024x1024 PNG) for zen-markdown.

Draws a rounded-rect macOS-style tile with a Geist Mono "z" in the
accent blue and a small magenta caret, on the Tokyo Night background.
"""
import sys
from PIL import Image, ImageDraw, ImageFont

OUT = sys.argv[1] if len(sys.argv) > 1 else "icon-1024.png"
FONT_PATH = sys.argv[2] if len(sys.argv) > 2 else "assets/fonts/GeistMono.ttf"

SIZE = 1024
MARGIN = 96  # macOS icons leave breathing room around the tile
RADIUS = 230

BG = (0x1A, 0x1B, 0x26, 255)
BG_EDGE = (0x12, 0x12, 0x1A, 255)
BLUE = (0x7A, 0xA2, 0xF7, 255)
MAGENTA = (0xBB, 0x9A, 0xF7, 255)
CYAN = (0x7D, 0xCF, 0xFF, 255)

img = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
draw = ImageDraw.Draw(img)

# vertical gradient tile
tile = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
tdraw = ImageDraw.Draw(tile)
inner = (MARGIN, MARGIN, SIZE - MARGIN, SIZE - MARGIN)
grad = Image.new("RGBA", (1, SIZE), (0, 0, 0, 0))
for y in range(SIZE):
    t = y / SIZE
    r = int(BG[0] * (1 - t) + BG_EDGE[0] * t)
    g = int(BG[1] * (1 - t) + BG_EDGE[1] * t)
    b = int(BG[2] * (1 - t) + BG_EDGE[2] * t)
    grad.putpixel((0, y), (r, g, b, 255))
grad = grad.resize((SIZE, SIZE))

mask = Image.new("L", (SIZE, SIZE), 0)
mdraw = ImageDraw.Draw(mask)
mdraw.rounded_rectangle(inner, radius=RADIUS, fill=255)
img.paste(grad, (0, 0), mask)

draw = ImageDraw.Draw(img)

# subtle inner stroke
draw.rounded_rectangle(inner, radius=RADIUS, outline=(0x29, 0x2E, 0x42, 255), width=4)

# the "z"
font = ImageFont.truetype(FONT_PATH, 620)
text = "z"
bbox = draw.textbbox((0, 0), text, font=font)
tw = bbox[2] - bbox[0]
th = bbox[3] - bbox[1]
tx = (SIZE - tw) / 2 - bbox[0]
ty = (SIZE - th) / 2 - bbox[1] - 40
draw.text((tx, ty), text, font=font, fill=BLUE)

# magenta caret under the z (markdown nod)
cx = SIZE / 2
cy = ty + th + 120
w = 150
draw.line([(cx - w, cy), (cx, cy + 70), (cx + w, cy)], fill=MAGENTA, width=34, joint="curve")

img.save(OUT)
print(f"wrote {OUT}")
