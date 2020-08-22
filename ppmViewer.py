from PIL import Image

with open('images/out100.ppm', 'rb') as f:
    im = Image.open(f)
    im.show()