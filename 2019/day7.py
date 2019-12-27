import utils
from itertools import permutations

def run_program(_program, *_input, phases = None):
    amp_ints = {amp:0 for amp in 'ABCDE'}
    amps = list('ABCDE')
    amp_ptr = 0
    amp = amps[amp_ptr]
    program = [i for i in _program]
    instruction_pointer = amp_ints['A']
    input_ptr = 0
    if phases:
        use_input_flag = True
        _input = _input[0]
        phases_ptr = 0
        programs = {amp:[i for i in _program] for amp in amps}
        program = programs['A']
    outputs = []
    while True:
        instruction = program[instruction_pointer]
        opcode = instruction % 100
        parameter_modes = str(instruction)[:-2]
        if opcode in [1,2,7,8]:
            instruction_jump = 4
            parameter_modes = f"{parameter_modes:0>3}"[::-1]
            parameters = program[instruction_pointer+1:instruction_pointer+4]
            values = [None] * 3
            for i, mode in enumerate(parameter_modes[:-1]):
                if mode == '0':
                    values[i] = program[parameters[i]]
                elif mode == '1':
                    values[i] = parameters[i]
                else:
                    raise ValueError(f'Parameter mode {mode} not understood.')
            dest = parameters[2]
            if opcode in [1,2]:
                val = values[0]+values[1] if opcode == 1 else values[0]*values[1]
            elif opcode == 7:
                val = 1 if values[0] < values[1] else 0
            elif opcode == 8:
                val = 1 if values[0] == values[1] else 0
            program[dest] = val
            instruction_pointer += instruction_jump
        elif opcode in [3,4]:
            instruction_jump = 2
            if opcode == 3:
                dest = program[instruction_pointer+1]
                if phases:
                    if instruction_pointer == 0:
                        program[dest] = phases[phases_ptr]
                        phases_ptr += 1
                    elif use_input_flag:
                        program[dest] = _input
                        use_input_flag = False
                    else:
                        program[dest] = outputs[-1]
                else:
                    try:
                        program[dest] = _input[input_ptr]
                        input_ptr += 1
                    except IndexError:
                        program[dest] = outputs[-1]
            elif opcode == 4:
                parameter_mode = f"{parameter_modes:0>1}"
                val = program[instruction_pointer+1]
                if parameter_mode == '0':
                    outputs.append(program[val])
                elif parameter_mode == '1':
                    outputs.append(val)
                else:
                    raise ValueError(f'Parameter mode {parameter_mode} not understood.')
            instruction_pointer += instruction_jump
            if phases and opcode == 4:
                amp_ints[amp] = instruction_pointer
                amp_ptr = (amp_ptr+1) % len(amps)
                amp = amps[amp_ptr]
                instruction_pointer = amp_ints[amp]
                program = programs[amp]
        elif opcode in [5,6]:
            instruction_jump = 3
            parameter_modes = f"{parameter_modes:0>2}"[::-1]
            values = [None, None]
            parameters = program[instruction_pointer+1:instruction_pointer+3]
            for i, mode in enumerate(parameter_modes):
                if mode == '0':
                    values[i] = program[parameters[i]]
                elif mode == '1':
                    values[i] = parameters[i]
                else:
                    raise ValueError(f'Parameter mode {mode} not understood.')
            if opcode == 5:
                instruction_pointer = values[1] if values[0]!=0 else instruction_pointer+instruction_jump
            else:
                instruction_pointer = values[1] if values[0]==0 else instruction_pointer+instruction_jump
        elif opcode == 99:
            break
        else:
            raise ValueError(f'Opcode {opcode} not understood')
    return outputs, program

def run_phase_sequence(program, sequence, feedback=False):
    amp_input = 0
    if feedback:
        amp_input = run_program(program, amp_input, phases=sequence)[0][-1]
    else:
        for phase in sequence:
            amp_input = run_program(program, phase, amp_input)[0][-1]
    return amp_input

def find_max_signal(program,feedback=False):
    current_max = -1
    sequences = permutations(range(5,10), 5) if feedback else permutations(range(5), 5) 
    for sequence in sequences:
        signal = run_phase_sequence(program, sequence, feedback)
        if signal > current_max:
            current_max = signal
    return current_max

program = utils.get_input(7)
program = [int(i) for i in program[0].split(',')]
first_star = find_max_signal(program,feedback=False)
second_star = find_max_signal(program, feedback=True)

utils.print_result(first_star,second_star)
