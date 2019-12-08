import utils

def get_layer(n, i, stride = 150):
    return i[n*stride:(n+1)*stride]

def fewest_zeros(i):
    min_zeros = 1e5
    min_layer = -1
    for layer in range(100):
        l = get_layer(layer, i)
        zeros = l.count('0')
        if zeros < min_zeros:
            min_zeros = zeros
            min_layer = layer
    return min_zeros, min_layer

def checksum(message):
    min_layer = fewest_zeros(message)[1]
    min_layer = get_layer(min_layer, message)
    return min_layer.count('1') * min_layer.count('2')

def print_image(img):
    return '\n'.join([''.join(i) for i in img])

def build_picture(i, width, height):
    img = [['2' for _ in range(width)] for _ in range(height)]
    layers = len(i) // (height * width)
    idx = 0
    for l in range(layers):
        for h in range(height):
            for w in range(width):
                if img[h][w] != '2':
                    pass
                else:
                    img[h][w] = '|' if i[idx] == '1' else i[idx] if i[idx] == '2' else ' ' 
                idx += 1
    return img

message = utils.get_input(8)[0]

first_star = checksum(message)
second_star = '\n' + print_image(build_picture(message, 25, 6))

utils.print_result(first_star, second_star)