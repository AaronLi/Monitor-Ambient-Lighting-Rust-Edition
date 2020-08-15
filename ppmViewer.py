from PIL import Image

with open('images/out0.ppm') as f:
    im = Image.open(f)
    im.show()