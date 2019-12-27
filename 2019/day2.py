import utils
input = utils.get_input(2)
input = [int(i) for i in input[0].split(',')]


def run_program(ints, noun, verb):
    out = [i for i in ints]
    out[1] = noun
    out[2] = verb
    idx = 0
    while True:
        op_code = out[idx]
        val = None
        if op_code == 1:
            val = out[out[idx+1]] + out[out[idx+2]]
            out[out[idx+3]] = val
            idx +=4
        elif op_code == 2:
            val = out[out[idx+1]]*out[out[idx+2]]
            out[out[idx+3]] = val
            idx +=4
        elif op_code == 99:
            break
        else:
            raise Exception('Opcode not in (1,2,99)')
    return out[0]


first_star = run_program(input, 12, 2)

def find_noun_verb(ints, target):
    max_index = len(ints)-1
    for noun in range(max_index):
        for verb in range(max_index):
            f = run_program(ints, noun, verb)
            if f == target:
                return 100*noun + verb

second_star = find_noun_verb(input, 19690720)

print(f'First star answer: {first_star}.\nSecond star answer: {second_star}.')