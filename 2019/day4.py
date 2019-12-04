import utils
from collections import Counter
input = utils.get_input(4)

def is_valid(pwrd):
    password = str(pwrd)
    prev_digit = 0
    double_digits = False
    for digit in password:
        d = int(digit)
        if d < prev_digit:
            return False
        prev_digit = d
    digit_counts = Counter(password)
    if digit_counts.most_common()[0][1] > 1:
        double_digits = True
    return double_digits

input = [int(i) for i in input[0].split('-')]
valid_passwords = [i for i in range(*input) if is_valid(i)]

first_star = len(valid_passwords)

def is_valid2(pwrd):
    password = str(pwrd)
    digit_counts = Counter(password)
    counts = set(count[1] for count in digit_counts.most_common())
    if 2 in counts:
        return True
    return False

second_star = len([p for p in valid_passwords if is_valid2(p)])

utils.print_result(first_star, second_star)