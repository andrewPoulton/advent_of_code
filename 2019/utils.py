def get_input(day):
	return open(f'inputs/day{day}').read().strip().split('\n')

def print_result(*stars):
	print(f'First star answer: {stars[0]}.\nSecond star answer: {stars[1]}.')