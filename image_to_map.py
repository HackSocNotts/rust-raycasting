from PIL import Image

image = Image.open("map.png")

for y in range(0, image.height):
    print("[")
    for x in range(0, image.width):
        pixel = image.getpixel((x, y))
        tile = ""

        match pixel:
            case (255, 0, 0):
                tile = "Tile::Wall(Color::RED),"
            case (0, 255, 0):
                tile = "Tile::Wall(Color::GREEN),"
            case (0, 0, 255):
                tile = "Tile::Wall(Color::BLUE),"
            case (0, 0, 0):
                tile = "Tile::Floor,"
            case _:
                continue

        print(tile, end="")
    print("],")
