import utils
import intcode
import curses


def print_screen(stdscr, buffer, *args, steps = None, minutes = None):
    for i, line in enumerate(buffer):
        stdscr.addstr(i, 0, ' '.join(line))
    if not steps is None:
        stdscr.addstr(i+1, 0, f'Steps taken: {steps}')
    if not minutes is None:
        stdscr.addstr(i+2, 0, f'Minutes elapsed: {minutes}')
    for k, arg in enumerate(args):
        stdscr.addstr(i+3+k, 0, arg)
    stdscr.refresh()

def is_empty(loc, buffer):
    return buffer[loc[1]][loc[0]] in ' .'

def get_border(loc):
    return [tuple((loc[0]+i, loc[1]+j)) for i,j in zip((0, 0, -1, 1), (-1,1,0,0))]

def on_border(loc, buffer):
    return any([is_empty(l, buffer) for l in get_border(loc)])


def spread_oxygen(border_oxygen, buffer, minutes):
    to_remove = []
    to_add = []
    for oxygen in border_oxygen:
        border = [b for b in get_border(oxygen) if is_empty(b, buffer)]
        for loc in border:
            buffer[loc[1]][loc[0]] = 'O'
            to_add.append(loc)
        to_remove.append(oxygen)
    for oxygen in to_remove:
        border_oxygen.remove(oxygen)
    for oxygen in to_add:
        border_oxygen.add(oxygen)
    minutes += 1 if border_oxygen else 0
    return buffer, border_oxygen, minutes

def fill_wall(loc, buffer, ic):
    border = get_border(loc)
    fill_outputs = []
    for _input, loc in zip([1,2,3,4], border):
        ic.run([_input])
        fill_outputs.append(ic.output[-1])
        if ic.output[-1] == 0:
            buffer[loc[1]][loc[0]] = '#'
        else:
            reverse = [0,2,1,4,3]
            ic.run([reverse[_input]])
    return buffer, fill_outputs

def ai(prev_attempt, prev_response, loc, buffer):
    border = get_border(loc)
    border_tiles = [buffer[l[1]][l[0]] for l in border]
    if prev_response > 0:
        newdir = prev_attempt
    else:
        try:
            newdir = border_tiles.index(' ') +  1
        except ValueError:
            reverse = [0,2,1,4,3]
            newdir = reverse[prev_attempt]
    return newdir, border_tiles, prev_attempt, prev_response
        

def run(stdscr, map_shape = (42,42), start= (21,21)):
    ic = intcode.Intcode(program)
    buffer = [[' ' for _ in range(map_shape[0])] for _ in range(map_shape[1])]
    print_screen(stdscr, buffer, steps = 0)
    bot_loc = start
    steps = 0
    status = 1
    x = 1
    prev_x = 1
    new_loc_flag = True
    oxygen_loc  = None
    oxygen_flag = False
    fill_outputs = None
    AI = False
    directions = [0, (0,-1), (0,1), (-1,0), (1,0)]
    border_tiles = None
    prev_attempt, prev_response = None,None
    while True:
        x = stdscr.getch()
        buffer[start[1]][start[0]] = '\u2588'
        key = x
        try:
            if x == curses.KEY_UP:
                x=1
            elif x == curses.KEY_DOWN:
                x=2
            elif x == curses.KEY_LEFT:
                x=3
            elif x == curses.KEY_RIGHT:
                x=4
            elif x == ord('q'):
                return 'user quit'
            elif x == ord('r'):
                steps = 0
            elif x == ord('o'):
                oxygen_flag = True
                if oxygen_loc is None:
                    oxygen_loc = start
                border_oxygen = set([oxygen_loc])
                minutes = 0
            elif x == ord('a'):
                AI = True
            elif x == ord('i'):
                AI = False
            else:
                print_screen(stdscr, buffer, steps=steps)
                continue
        except:
            continue

        if oxygen_flag:
            buffer, border_oxygen, minutes = spread_oxygen(border_oxygen, buffer, minutes)
            print_screen(stdscr, buffer, f'Spreading oxygen, starting from location {oxygen_loc}', minutes=minutes)

        elif x in [1,2,3,4]:
            if AI:
                prev_x, border_tiles, prev_attempt, prev_response = ai(prev_x, status, bot_loc, buffer)
                x=prev_x
            else:
                prev_x = x
            direction = directions[x]
            return_code = ic.run([x])
            if return_code == 0:
                return 'halted apparently'
            else:
                status = ic.output[-1]
                if status == 0:
                    buffer[bot_loc[1]+direction[1]][bot_loc[0]+direction[0]] = '#'
                elif status == 1:
                    new_bot_loc = (bot_loc[0] + direction[0], bot_loc[1]+direction[1])
                    old_bot_loc = (bot_loc[0] - direction[0], bot_loc[1]-direction[1])

                    if buffer[new_bot_loc[1]][new_bot_loc[0]] == ' ':
                        steps += 1
                        new_loc_flag = True
                    elif (buffer[new_bot_loc[1]][new_bot_loc[0]] == '.') or (new_bot_loc == start):
                        steps -= 1
                        new_loc_flag = False

                    if buffer[bot_loc[1]][bot_loc[0]] == '\u0416':
                        pass
                    else:
                        buffer[bot_loc[1]][bot_loc[0]] = '.' if new_loc_flag else ' '
                    
                    bot_loc = new_bot_loc
                    if buffer[bot_loc[1]][bot_loc[0]] == '\u0416':
                        pass
                    else:
                        buffer[bot_loc[1]][bot_loc[0]] = 'D'
                    buffer, fill_outputs = fill_wall(bot_loc, buffer, ic)


                elif status == 2:
                    buffer[bot_loc[1]][bot_loc[0]] = '.'
                    bot_loc = (bot_loc[0] + direction[0], bot_loc[1]+direction[1])
                    buffer[bot_loc[1]][bot_loc[0]] = '\u0416'
                    oxygen_loc = bot_loc
                    steps +=1        
            print_screen(stdscr, buffer, f'debug: ai attempt={x}, status:{status}, border_tiles: {border_tiles}, prev_attempt: {prev_attempt}, {prev_response}', steps=steps)
    return buffer

if __name__ == "__main__":
    program = utils.get_input(15)
    program = [int(i) for i in program[0].split(',')]
    curses.wrapper(run)

