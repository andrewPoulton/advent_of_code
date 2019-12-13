import utils
from intcode import Intcode


def first_star_answer(program):
    ic = Intcode(program)
    ic.run()
    output = [c for i, c in enumerate(ic.output) if (i%3==2) and (c==2)]
    return len(output)

def set_memory(ic, loc, val):
    ic.program[loc] = val

def use_joystick(ic, joystick):
    assert ic.awaiting_input
    return ic.run([joystick])

def init_screen(ic):
    x_loc = [x for i,x in enumerate(ic.output) if i%3==0]
    y_loc = [y for i,y in enumerate(ic.output) if i%3==1]
    screen_buffer = [[' ' for _ in range(max(x_loc)+1)] for _ in range(max(y_loc)+1)]
    return screen_buffer

def print_screen(buffer, score):
    print('\n'.join(' '.join(x) for x in buffer) + f' {score}')

def parse_output(output, screen_buffer, score, paddle_loc = None, ball_loc = None):
    assert len(output) % 3 == 0
    x_loc = [x for i,x in enumerate(output) if i%3==0]
    y_loc = [y for i,y in enumerate(output) if i%3==1]
    things = [z for i,z in enumerate(output) if i%3==2]
    for x,y,thing in zip(x_loc,y_loc,things):
        if (x == -1) and (y == 0):
            score = thing
        if thing == 0:
            screen_buffer[y][x] = ' '
        elif thing == 1:
            screen_buffer[y][x] = '\u2588'
        elif thing == 2:
            screen_buffer[y][x] = '\u2591'
        elif thing == 3:
            screen_buffer[y][x] = '\u2581'
            paddle_loc = (x,y)
        elif thing == 4:
            screen_buffer[y][x] = '\u001b[31m\u2588\u001b[0m'
            ball_loc = (x,y)
    return score, screen_buffer, ball_loc, paddle_loc

def run_frame(ic, joystick, buffer, score, paddle_loc, ball_loc):
    use_joystick(ic, joystick)
    score, buffer, ball_loc, paddle_loc = parse_output(ic.output, buffer, score, paddle_loc, ball_loc)
    ic.output = []
    return score, buffer, ball_loc, paddle_loc

def mark_ball(screen_buffer, ball_loc, old_loc):
    screen_buffer[ball_loc[1]][0] = '\u001b[31m\u2588\u001b[0m'
    screen_buffer[ball_loc[1]][-1] = '\u001b[31m\u2588\u001b[0m'
    screen_buffer[0][ball_loc[0]] = '\u001b[31m\u2588\u001b[0m'
    screen_buffer[old_loc[1]][0] = '\u001b[37m\u2588\u001b[0m'
    screen_buffer[old_loc[1]][-1] = '\u001b[37m\u2588\u001b[0m'
    screen_buffer[0][old_loc[0]] = '\u001b[37m\u2588\u001b[0m'
    return screen_buffer

def play(ic, _input = None, _print=True):
    from inputimeout import inputimeout, TimeoutOccurred
    ic.reset()
    ic.program[0] = 2
    ic.run()
    assert ic.awaiting_input
    buffer = init_screen(ic)
    score = 0
    score, buffer, ball_loc, paddle_loc = parse_output(ic.output, buffer, score)
    if _print:
        print_screen(buffer, score)
    ic.output = []
    auto = False
    auto_count = -1
    duration = 0
    while True:
        joystick = -1 if paddle_loc[0] > ball_loc[0] else 1 if paddle_loc[0] < ball_loc[0] else 0
        if (not auto) and _print:
            x = input('')
            if x == 'a':
                joystick = -1
            elif x == 'd':
                joystick = 1
            elif x == 'w':
                auto = True
                duration = int(input())
                auto_count = 0
        elif auto_count == duration:
            auto = False
        else:
            auto_count += 1
        new_score, buffer, new_ball_loc, paddle_loc = run_frame(ic, joystick, buffer, score, paddle_loc, ball_loc)
        if new_ball_loc == None:
            return new_score
        buffer = mark_ball(buffer, new_ball_loc, ball_loc)
        ball_loc = new_ball_loc
        if _print:
            print_screen(buffer, new_score)
        if ic.halted:
            return new_score
        score = new_score

if __name__ == "__main__":
    program = utils.get_input(13)
    program = [int(i) for i in program[0].split(',')]
    first_star = first_star_answer(program)
    ic = Intcode(program)
    second_star = play(ic, _print=False)
    utils.print_result(first_star, second_star)