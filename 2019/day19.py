import utils
import intcode

program = utils.get_input(19)
program = [int(i) for i in program[0].split(',')]
ic = intcode.Intcode(program, mem_size = 4000)
map = ''
beam_count = 0
for y in range(100):
    for x in range(100):
        ic.run([x,y])
        if ic.output[-1] == 1:
            map += '#'
            if (x<50) and (y<50):
                beam_count +=1
        else:
            map+='.'
        ic.reset()
    map += '\n'
print(map)