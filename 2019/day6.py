import utils

def build_orbit_tree(orbits):
    orbit_tree = {'COM':{'p':None, 'c':[]}}
    for orbit in orbits:
        major, minor = orbit.split(')')
        if not major in orbit_tree:
            orbit_tree[major] = {'p':'COM', 'c':[minor]}
        else:
            orbit_tree[major]['c'].append(minor)
        if not minor in orbit_tree:
            orbit_tree[minor] = {'p':major, 'c':[]}
        else:
            orbit_tree[minor]['p'] = major
    return orbit_tree

def traverse_tree(tree):
    node = tree['COM']
    directs = 0
    indirects = 0
    to_revisit = [(tree[n],0) for n in node['c']]
    visited = set(['COM'])
    while to_revisit:
        node, level = to_revisit.pop()
        to_revisit += [(tree[n], level+1) for n in node['c']]
        directs +=1
        indirects += level
    return directs, indirects

orbit_map = utils.get_input(6)
orbit_tree = build_orbit_tree(orbit_map)

first_star = sum(traverse_tree(orbit_tree))

def min_dist(me, santa, tree):
    my_node = tree[tree[me]['p']]
    santa_node = tree[tree[santa]['p']]
    my_path = {tree[me]['p']:0}
    santa_path = {tree[santa]['p']:0}
    step_count = 1
    while True:
        my_next = my_node['p']
        santa_next = santa_node['p']
        if my_next:
            my_path[my_next] = step_count
            my_node = tree[my_next]
        if santa_next:
            santa_path[santa_next] = step_count
            santa_node = tree[santa_next]
        if my_next in santa_path:
            break
        step_count += 1
    return my_path, santa_path, my_next

my_path, santa_path, nearest_common = min_dist('YOU', 'SAN', orbit_tree)
second_star = my_path[nearest_common] + santa_path[nearest_common]

utils.print_result(first_star, second_star)
