import utils
import numpy as np

def pos2dict(pos):
    pos = pos[1:-1].split(', ')
    return {axis:int(val) for axis, val in [p.split('=') for p in pos]}

def get_dynamics(_input):
    positions = [pos2dict(i) for i in _input]
    velocities = [{'x':0,'y':0,'z':0} for _ in positions]
    return {'p':positions, 'v':velocities}

def update_postion(position, velocity):
    for p in position:
        position[p] += velocity[p]
    return position

def apply_gravity(dynamics):
    v = dynamics['v']
    for i, position1 in enumerate(dynamics['p'][:-1]):
        for j, position2 in enumerate(dynamics['p'][i+1:]):
            for p in position1:
                if position1[p] > position2[p]:
                    v[i][p] -= 1
                    v[j+i+1][p] += 1
                elif position1[p] < position2[p]:
                    v[i][p] += 1
                    v[j+i+1][p] -= 1
    return dynamics

def step(dynamics):
    dynamics = apply_gravity(dynamics)
    dynamics['p'] = [update_postion(p,v) for p,v in zip(dynamics['p'], dynamics['v'])]
    return dynamics

def calculate_energy(dynamics):
    E = 0
    for p, v in zip(dynamics['p'], dynamics['v']):
        PE = sum([abs(val) for val in p.values()])
        KE = sum([abs(val) for val in v.values()])
        E += PE*KE
    return E

def first_star_answer(dynamics, steps = 1000):
    for _step in range(steps):
        dynamics = step(dynamics)
    E = calculate_energy(dynamics)
    return E

def get_state(dynamics, axis):
    return [p[axis] for p in dynamics['p']].__repr__() + [p[axis] for p in dynamics['v']].__repr__()

def second_star_answer(_input):
    dynamics = get_dynamics(_input)
    states = {'x':set(), 'y':set(), 'z':set()}
    rep_x = get_state(dynamics, 'x')
    rep_y = get_state(dynamics, 'y')
    rep_z = get_state(dynamics, 'z')
    states['x'].add(rep_x)
    states['y'].add(rep_y)
    states['z'].add(rep_z)
    axis_periods = [0,0,0]
    steps = 0
    while True:
        dynamics = step(dynamics)
        steps+=1
        states_revisited = 0
        for i, (period, axis) in enumerate(zip(axis_periods, 'xyz')):
            if period > 0:
                states_revisited += 1
                continue
            else:
                rep = get_state(dynamics, axis)
                if rep in states[axis]:
                    axis_periods[i] = steps
        if states_revisited == 3:
            break
    return axis_periods

_input = utils.get_input(12)
dynamics= get_dynamics(eg)
first_star = first_star_answer(dynamics)
second_stars = second_star_answer(eg)
second_star = np.lcm(np.lcm(*second_stars[:2]), second_stars[2])
utils.print_result(first_star, second_star)