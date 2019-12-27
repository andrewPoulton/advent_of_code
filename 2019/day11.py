import utils
from intcode import Intcode


def move(current_loc, facing, turn_direction): 
    f = 'urdl' 
    fidx = f.index(facing) 
    new_idx = (fidx+1 if turn_direction else fidx-1) % 4 
    new_facing = f[new_idx] 
    if new_facing == 'u': 
        new_loc = (current_loc[0], current_loc[1] + 1) 
    elif new_facing == 'r':
        new_loc = (current_loc[0]+1, current_loc[1])
    elif new_facing == 'd':
        new_loc = (current_loc[0], current_loc[1] - 1) 
    elif new_facing == 'l':
        new_loc = (current_loc[0]-1, current_loc[1])
    return new_loc, new_facing

def operate_robot(computer, start_colour = 0):
    current_loc = (0,0)
    facing = 'u'
    panels = {(0,0):start_colour}
    i = 0
    while i <10000:
        _input = panels[current_loc]
        return_code = computer.run([_input])
        if return_code == 0:
            break
        # else:
        #     # pass
        #     print(computer.input_ptr, computer.ptr)
        for j, output in enumerate(computer.output):
            if j % 2 == 0:
                panels[current_loc] = output
            else:
                current_loc, facing = move(current_loc, facing, output)
        computer.output = []
        if not current_loc in panels:
            panels[current_loc] = 0
        i += 1
    return panels, current_loc, facing, i

def painted_image(panels):
    x_min = min([k for k in panels], key = lambda x:x[0])[0]
    y_min = min([k for k in panels], key = lambda x:x[1])[1]
    x_max = max([k for k in panels], key = lambda x:x[0])[0]
    y_max = max([k for k in panels], key = lambda x:x[1])[1]
    x = x_max-x_min
    y = y_max-y_min
    picture = [['.' for _ in range(x+1)] for _ in range(y+1)]
    for panel, colour in panels.items():
        picture[y_max-panel[1]][panel[0]-x_min] = '\u2588' if colour else '.'
    return '\n'.join(' '.join(p) for p in picture)

program = utils.get_input(11)
program = [int(i) for i in program[0].split(',')]

computer = Intcode(program)

first_star = len(operate_robot(computer, start_colour=0)[0])
computer.reset()
second_star = '\n' + painted_image(operate_robot(computer, start_colour=1)[0])
utils.print_result(first_star, second_star)