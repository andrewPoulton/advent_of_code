import utils
import numpy as np

def compute_pattern(_input, idx):
    base_pattern = [0,1,0,-1]
    pattern = np.repeat(base_pattern, idx+1)
    length_diff = len(_input)//len(pattern)
    pattern = np.tile(pattern, length_diff +1)
    return pattern[1: len(_input)+1]

def compute_value(_input, pattern):
    val = (_input * pattern).sum()
    return abs(val) % 10


    

def input_to_vec(_input):
    return np.array([int(i) for i in _input])

def all_patterns(_input):
    patterns = ['' for _ in range(len(_input))]
    for idx in range(len(_input)):
        patterns[idx] = np.array(compute_pattern(_input, idx))
    return patterns

def phase(_input, patterns):
    output = np.zeros_like(_input)
    for idx, pattern in enumerate(patterns):
        val = compute_value(_input, pattern)
        output[idx] = val
    return output

def multi_phases(_input, phases=100):
    patterns = all_patterns(_input)
    output = input_to_vec(_input)
    for _ in range(phases):
        output = phase(output, patterns)
    return output

def first_star_answer(_input):
    output = multi_phases(_input, 100)
    return ''.join([str(x) for x in output[:8].tolist()])