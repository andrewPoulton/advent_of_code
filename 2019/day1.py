import utils

def mass_to_fuel(mass):
    mass = int(mass)
    return max(0,int(mass/3) - 2)

def additional_fuel_mass(mass):
    total_mass = 0
    mass = int(mass)
    while mass > 0:
        new_mass = mass_to_fuel(mass)
        total_mass += new_mass
        mass = new_mass
    return total_mass

input = utils.get_input(1)
first_star = sum([mass_to_fuel(mass) for mass in input])
second_star = sum([additional_fuel_mass(mass) for mass in input])
print(f'First star answer: {first_star}.\nSecond star answer: {second_star}.')
