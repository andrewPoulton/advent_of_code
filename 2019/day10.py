import utils
from collections import Counter
from itertools import product

def map_to_locs(asteroid_map):
    h = len(asteroid_map)
    w = len(asteroid_map[0])
    return [(x,y) for y,x in product(range(h), range(w)) if asteroid_map[y][x] == '#']

def los(s,e):
    slope = (e[0]-s[0],e[1]-s[1])
    return slope

def normalize(direction):
    norm = (direction[0]**2 + direction[1]**2)**.5
    return (direction[0]/norm, direction[1]/norm) if norm else direction

def colinear(norm1, norm2):
    x_delta = abs(norm1[0] - norm2[0])
    y_delta = abs(norm1[1] - norm2[1])
    eps = 1e-5
    return (x_delta < eps) & (y_delta < eps)

def get_unseeable(asteroid, asteroids):
    directions = [normalize(los(asteroid, a)) for a in asteroids]
    seeable = [True for _ in directions]
    norms = [to_string(norm) for norm in directions]
    norm_counts = Counter(norms)
    mults = [key for key, val in norm_counts.items() if val > 1]
    cant_see = []
    for mult in mults:
        candidates = [i for i, norm in enumerate(norms) if norm == mult]
        cant_see += [asteroids[i] for i in candidates[1:]]
    return cant_see

def get_seeable(asteroid, asteroids):
    directions = [normalize(los(asteroid, a)) for a in asteroids]
    seeable = [True for _ in directions]
    norms = [to_string(norm) for norm in directions]
    norm_counts = Counter(norms)
    mults = [key for key, val in norm_counts.items() if val > 1]
    cant_see = []
    for mult in mults:
        candidates = [i for i, norm in enumerate(norms) if norm == mult]
        cant_see += [asteroids[i] for i in candidates[1:]]
    cant_see = set(cant_see)
    can_see = [asteroid for asteroid in asteroids if not asteroid in cant_see]
    return can_see

def to_string(norm):
    return f'({norm[0]:.5f}, {norm[1]:.5f})'

def clockwise_sort(norms):
    norms = [(norm, i) for i, norm in enumerate(norms)]
    lower_left = [norm for norm in norms if (norm[0][0] <0.) and (norm[0][1] >=0.)] 
    upper_left = [norm for norm in norms if (norm[0][0] <0.) and (norm[0][1] <0.)] 
    upper_right = [norm for norm in norms if (norm[0][0] >=0.) and (norm[0][1] <= 0.)] 
    lower_right = [norm for norm in norms if (norm[0][0] >=0.) and (norm[0][1] >0.)] 
    sort_func = lambda x: x[0][0]
    return sorted(upper_right,key=sort_func,  ) + \
            sorted(lower_right,key=sort_func,reverse=True) + \
            sorted(lower_left,key = sort_func, reverse=True) + \
            sorted(upper_left, key=sort_func,)

def first_star_(asteroids):
    unseeable = {asteroid:[] for asteroid in asteroids}
    for i, asteroid in enumerate(asteroids):
        cant_see = get_unseeable(asteroid, asteroids[i:])
        unseeable[asteroid] += cant_see
        for a in cant_see:
            unseeable[a].append(asteroid)
    seeable = {a:len(unseeable)-len(unseeable[a])-1 for a in unseeable} 
    return seeable, unseeable

asteroid_map = utils.get_input(10)
asteroid_map = [list(line) for line in asteroid_map]

asteroids = map_to_locs(asteroid_map)
seeable, unseeable = first_star_(asteroids)
best_asteroid, first_star = max([(key, val) for key, val in seeable.items()], key = lambda x:x[1])

visible_from_best = [asteroid for asteroid in asteroids \
                    if (not asteroid in unseeable[best_asteroid]) \
                    and (asteroid != best_asteroid)]

visible_from_best_directions = [normalize(los(best_asteroid, asteroid)) for asteroid in visible_from_best]

visible_sorted = clockwise_sort(visible_from_best_directions)
vaporised_200th = visible_from_best[visible_sorted[199][1]]
second_star = 100*vaporised_200th[0] + vaporised_200th[1]
utils.print_result(first_star, second_star)
