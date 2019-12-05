import utils

def run_program(_program, _input):
    program = [i for i in _program]
    instruction_pointer = 0
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
                program[dest] = _input
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
        

                    
inputs = utils.get_input(5)
program = [int(i) for i in inputs[0].split(',')]
first_star = run_program(program, 1)[0][-1]
second_star = run_program(program, 5)[0][-1]
utils.print_result(first_star, second_star)
                
            
