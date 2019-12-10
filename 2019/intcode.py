class Intcode:
    def __init__(self, program, mem_size = 1000):
        self.program = [i for i in program]
        self._input = []
        self.ptr = 0
        self.relative_base = 0
        self.input_ptr = 0
        self.halted = False
        self.awaiting_input = False
        self.output = []
        self.extended_memory = [0 for _ in range(mem_size)]
        self.program.extend(self.extended_memory)
        self._program = program
        available_ops = [(op, op.split('_')[1]) for op in self.__dir__() if 'opcode' in op]
        self.op_dict = {int(code) : self.__getattribute__(op) for op, code in available_ops}

    def reset(self):
        self.__init__(self._program)

    @staticmethod
    def parse_instruction(instruction, *parameters):
        opcode = instruction % 100
        parameter_modes = str(instruction)[:-2]
        parameter_modes = f"{parameter_modes:0>{len(parameters)}}"[::-1]
        num_params = 0
        is_write_param = []
        if opcode in [1,2,7,8]:
            is_write_param = [False,False,True]
            num_params = 3
        elif opcode == 3:
            is_write_param = [True]
            num_params = 1
        elif opcode == 4:
            is_write_param = [False]
            num_params = 1
        elif opcode in [5,6]:
            is_write_param = [False, False]
            num_params = 2
        elif opcode == 9:
            is_write_param = [False]
            num_params = 1
        parameters = parameters[:num_params]
        parameter_modes = parameter_modes[:num_params]
        return opcode, list(zip(parameter_modes, parameters, is_write_param))

    def interpret_parameters(self, modes_and_parameters):
        values = []
        for mode, param, is_write_param in modes_and_parameters:
            if mode == '0':
                val = param if is_write_param else self.program[param]
            elif mode == '1':
                val = param
            elif mode == '2':
                val = param+self.relative_base if is_write_param else self.program[param + self.relative_base]
            else:
                raise ValueError(f'Parameter mode {mode} not understood.')
            values.append(val)
        return values
            
    def opcode_1(self, parameters):
        values = self.interpret_parameters(parameters)
        val = values[0] + values[1]
        self.program[values[2]] = val
        self.ptr += 4

    def opcode_2(self, parameters):
        values = self.interpret_parameters(parameters)
        val = values[0] * values[1]
        self.program[values[2]] = val
        self.ptr += 4
    
    def opcode_3(self, parameters):
        values = self.interpret_parameters(parameters[:1])
        try:
            val = self._input[self.input_ptr]
            self.input_ptr += 1
            self.program[values[0]] = val
            self.ptr += 2
        except IndexError:
            self.awaiting_input = True
        
    def opcode_4(self, parameters):
        values = self.interpret_parameters(parameters[:1])
        self.output.append(values[0])
        self.ptr += 2

    def opcode_5(self, parameters):
        values = self.interpret_parameters(parameters[:2])
        if values[0] != 0:
            self.ptr = values[1]
        else:
            self.ptr += 3

    def opcode_6(self, parameters):
        values = self.interpret_parameters(parameters[:2])
        if values[0] == 0:
            self.ptr = values[1]
        else:
            self.ptr += 3

    def opcode_7(self, parameters):
        values = self.interpret_parameters(parameters)
        val = 1 if values[0] < values[1] else 0
        self.program[values[2]] = val
        self.ptr += 4

    def opcode_8(self, parameters):
        values = self.interpret_parameters(parameters)
        val = 1 if values[0] == values[1] else 0
        self.program[values[2]] = val
        self.ptr += 4

    def opcode_9(self, parameters):
        values = self.interpret_parameters(parameters[:1])
        self.relative_base += values[0]
        self.ptr += 2

    def opcode_99(self, parameters):
        self.halted = True

    def run(self, _input = None):
        if not _input is None:
            self._input += _input
        while not self.halted:
            params = self.program[self.ptr:self.ptr+4]
            opcode, parameters = self.parse_instruction(*params)
            self.op_dict[opcode](parameters)



if __name__ == "__main__":
    import utils
    program = utils.get_input(9)
    program = [int(i) for i in program[0].split(',')]

    computer = Intcode(program)
    computer.run([1])
    first_star = computer.output[0]
    computer.reset()
    computer.run([2])
    second_star = computer.output[0]
    utils.print_result(first_star, second_star)
