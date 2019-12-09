import utils
from itertools import permutations
import pdb

def run_program(_program, *_input, phases = None):
    #initialize
    program = [i for i in _program]
    extended_memory = [0 for _ in range(10000)]
    program+=extended_memory
    instruction_pointer = 0
    input_ptr = 0
    relative_base = 0
    opcodes = []
    if phases:
        amp_ints = {amp:0 for amp in 'ABCDE'}
        amps = list('ABCDE')
        amp_ptr = 0
        amp = amps[amp_ptr]
        use_input_flag = True
        _input = _input[0]
        phases_ptr = 0
        programs = {amp:[i for i in _program] for amp in amps}
        program = programs['A']
    outputs = []

    while True:
        instruction = program[instruction_pointer]
        opcode = instruction % 100
        opcodes.append(instruction)
        parameter_modes = str(instruction)[:-2]
        if opcode in [1,2,7,8]:
            instruction_jump = 4
            parameter_modes = f"{parameter_modes:0>{instruction_jump-1}}"[::-1]
            parameters = program[instruction_pointer+1:instruction_pointer+4]
            values = [None] * 3
            for i, mode in enumerate(parameter_modes[:-1]):
                if mode == '0':
                    values[i] = program[parameters[i]]
                elif mode == '1':
                    values[i] = parameters[i]
                elif mode == '2':
                    values[i] = program[parameters[i] + relative_base]
                else:
                    raise ValueError(f'Parameter mode {mode} not understood.')
            dest = parameters[2]
            dest += relative_base if parameter_modes[-1] == '2' else 0
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
                parameter_mode = f"{parameter_modes:0>{instruction_jump-1}}"[::-1]
                parameter = program[instruction_pointer+1]
                if parameter_mode == '0':
                    dest = parameter
                elif parameter_mode == '2':
                    dest = parameter + relative_base
                else:
                    raise ValueError(f'Parameter mode {parameter_mode} not recognised for opcode {opcode}')
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
                parameter_mode = f"{parameter_modes:0>{instruction_jump-1}}"[::-1]
                val = program[instruction_pointer+1]
                if parameter_mode == '0':
                    val = program[val]
                elif parameter_mode == '1':
                    val = val
                elif parameter_mode == '2':
                    val = program[val+relative_base]
                else:
                    raise ValueError(f'Parameter mode {parameter_mode} not understood.')
                outputs.append(val)
            instruction_pointer += instruction_jump
            if phases and opcode == 4:
                amp_ints[amp] = instruction_pointer
                amp_ptr = (amp_ptr+1) % len(amps)
                amp = amps[amp_ptr]
                instruction_pointer = amp_ints[amp]
                program = programs[amp]
        elif opcode in [5,6]:
            instruction_jump = 3
            parameter_modes = f"{parameter_modes:0>{instruction_jump-1}}"[::-1]
            values = [None, None]
            parameters = program[instruction_pointer+1:instruction_pointer+3]
            for i, mode in enumerate(parameter_modes):
                if mode == '0':
                    values[i] = program[parameters[i]]
                elif mode == '1':
                    values[i] = parameters[i]
                elif mode == '2':
                    values[i] = program[parameters[i] + relative_base]
                else:
                    raise ValueError(f'Parameter mode {mode} not understood.')
            if opcode == 5:
                instruction_pointer = values[1] if values[0]!=0 else instruction_pointer+instruction_jump
            else:
                instruction_pointer = values[1] if values[0]==0 else instruction_pointer+instruction_jump
        elif opcode == 9:
            instruction_jump = 2
            parameter_mode = f"{parameter_modes:0>{instruction_jump-1}}"[::-1]
            parameter = program[instruction_pointer+1]
            if parameter_mode == '1':
                parameter = parameter
            elif parameter_mode == '0':
                parameter = program[parameter]
            elif parameter_mode == '2':
                parameter = program[parameter + relative_base]
            relative_base += parameter
            instruction_pointer += instruction_jump

        elif opcode == 99:
            break
        else:
            raise ValueError(f'Opcode {opcode} not understood')
    return outputs, program[:-len(extended_memory)], opcodes

program = utils.get_input(9)
program = [int(i) for i in program[0].split(',')]
first_star = run_program(program,1)[0][0]
second_star = run_program(program,2)[0][0]
utils.print_result(first_star, second_star)