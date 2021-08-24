#!/usr/bin/env python3
import json
import random
from typing import Union

from PIL import Image
from enum import Enum

PATH = "./resources/map.png"
WIDTH = 300
HEIGHT = 150


class Color(Enum):
    RED = 1
    GREEN = 2
    BLUE = 3


def index_of_max(a: tuple):
    largest = max(a)
    return a.index(largest)


def get_color(pixel):
    max_index = index_of_max(pixel)
    if max_index == 0:
        color = Color.RED
    elif max_index == 1:
        color = Color.GREEN
    elif max_index == 2:
        color = Color.BLUE
    else:
        raise ValueError('')

    return color


tiles = {
    "grass": {
        "default": [
            "generic-rpg-tile70",
        ]
    },
    "water": {
        "default": [
            "generic-rpg-tile65",
        ],
        "top_grass": "generic-rpg-tile-waterfall06",
        "right_top_corner_grass": "generic-rpg-tile-waterfall05",
        "left_top_corner_grass": "generic-rpg-tile-waterfall07",
    },
    "road": {
        "default": [
            "generic-rpg-tile71",
        ],
        "grass": {
            "up": "generic-rpg-tile06",
            "down": "generic-rpg-tile44",
            "left": "generic-rpg-tile14",
            "right": "generic-rpg-tile48",
            "corner": {
                "up_right": "generic-rpg-tile33",
                "up_left": "generic-rpg-tile19",
                "down_right": "generic-rpg-tile62",
                "down_left": "generic-rpg-tile57",
            },
        }
    },
}


class PixelHelpers:
    def __init__(self, loaded_image):
        self.loaded_image = loaded_image

    def get_pixel(self, x, y):
        try:
            pixel = self.loaded_image[x, y]
        except IndexError:
            pixel = None

        return pixel

    def down(self, x, y) -> Union[tuple, None]:
        return self.get_pixel(x, y + 1)

    def up(self, x, y) -> Union[tuple, None]:
        return self.get_pixel(x, y - 1)

    def left(self, x, y) -> Union[tuple, None]:
        return self.get_pixel(x - 1, y)

    def right(self, x, y) -> Union[tuple, None]:
        return self.get_pixel(x + 1, y)


def get_sprite(center, left, right, up, down):
    sprite = ''

    if get_color(center) == Color.RED:
        sprite = random.choice(tiles["road"]["default"])
        # road
        if up and get_color(up) == Color.GREEN:
            sprite = tiles["road"]["grass"]["up"]
        elif down and get_color(down) == Color.GREEN:
            sprite = tiles["road"]["grass"]["down"]
        elif left and get_color(left) == Color.GREEN:
            sprite = tiles["road"]["grass"]["left"]
        elif right and get_color(right) == Color.GREEN:
            sprite = tiles["road"]["grass"]["right"]
    elif get_color(center) == Color.GREEN:
        sprite = random.choice(tiles["grass"]["default"])
    elif get_color(center) == Color.BLUE:
        sprite = random.choice(tiles["water"]["default"])
    return sprite


def iter_generate_map(loaded_image):
    pixel_helpers = PixelHelpers(loaded_image)

    for y in range(HEIGHT):
        for x in range(WIDTH):
            center = loaded_image[x, y]
            up = pixel_helpers.up(x, y)
            down = pixel_helpers.down(x, y)
            left = pixel_helpers.left(x, y)
            right = pixel_helpers.right(x, y)

            center_color = get_color(center)
            sprite = get_sprite(
                center=center,
                up=up,
                down=down,
                left=left,
                right=right,
            )

            yield {
                "pos_x": x,
                "pos_y": y,
                "color": center_color,
                "sprite": sprite,
            }


def load_image(path: str):
    return Image.open(path).convert("RGB").load()


def main():
    loaded_image = load_image(PATH)
    map_dict = {}
    for tile in iter_generate_map(loaded_image):
        pos_x = tile["pos_x"]
        pos_y = tile["pos_y"]
        map_dict.update({f"{pos_x}_{pos_y}": {"sprite": tile["sprite"]}})

    print(json.dumps(map_dict, indent=2))


if __name__ == '__main__':
    main()
