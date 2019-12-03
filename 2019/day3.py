import utils
i = utils.get_input(3)
i
t = ["R75,D30,R83,U83,L12,D49,R71,U7,L72","U62,R66,U55,R34,D71,R55,D58,R83"]
t=[i.split(',') for i in t]


def to_segments(paths):
    segments = []
    loc = [0,0]
    for i, path in enumerate(paths):
        d = path[0]
        dist = int(path[1:])
        if d == 'R':
            new_loc = [loc[0]+dist, loc[1]]
        elif d == 'L':
            new_loc = [loc[0]-dist, loc[1]]
        elif d == 'U':
            new_loc = [loc[0], loc[1]+dist]
        elif d == 'D':
            new_loc = [loc[0], loc[1]-dist]
        else:
            raise Exception(f'Unknown direction: {d}')
        segments.append((loc, new_loc, path))
        loc = new_loc
    return segments


def seg_intersect(s1, s2):
    d1 = s1[2][0]
    if d1 in ['U','D']:
        s2, s1 = s1, s2
        if s1[2][0] in ['U','D']:
            return -1
    if s2[2][0] in ['L', 'R']:
        return -1 
    s2x = s2[0][0]
    s1x = sorted([s1[0][0], s1[1][0]])
    s1y = s1[0][1]
    s2y = sorted([s2[0][1], s2[1][1]])
    if (s1x[0] <= s2x <= s1x[1]) & (s2y[0] <= s1y <= s2y[1]):
        return (s2x, s1[0][1])
    return -1


def intersection_points_with_steps(p1,p2):
    ip=[]
    steps = []
    for i, p in enumerate(p1):
        for j, q in enumerate(p2):
            intersect = seg_intersect(p,q)
            if intersect != -1:
                ip.append(intersect)
                steps.append([i,j])
    return ip, steps

input = utils.get_input(3)
input = [to_segments(i.split(',')) for i in input]

intersections, steps = intersection_points_with_steps(*input)

dists = [(abs(intersection[0])+abs(intersection[1])) for intersection in intersections]
first_star = min(dists)


def manhatten(p1, p2):
    return abs(p1[0] - p2[0]) + abs(p1[1] - p2[1])

def steps_to_intersection(step, intersection, segments):
    i, j = step
    first_wire_steps = sum([int(seg[2][1:]) for seg in segments[0][:i]])
    first_wire_steps += manhatten(segments[0][i][0], intersection)
    second_wire_steps = sum([int(seg[2][1:]) for seg in segments[1][:j]])
    second_wire_steps += manhatten(segments[1][j][0], intersection)
    return first_wire_steps + second_wire_steps
    
second_star = steps_to_intersection(steps[0], intersections[0], input)

utils.print_result(first_star, second_star)
