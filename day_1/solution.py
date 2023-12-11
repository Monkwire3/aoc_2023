sum_of_calibration_values = 0


str_values = {
        "1" : 1,
        "2" : 2,
        "3" : 3,
        "4" : 4,
        "5" : 5,
        "6" : 6,
        "7" : 7,
        "8" : 8,
        "9" : 9,
        "0" : 0,
        "one" : 1,
        "two" : 2,
        "three" : 3,
        "four" : 4,
        "five" : 5,
        "six" : 6,
        "seven" : 7,
        "eight" : 8,
        "nine" : 9,
        }


def get_first_int(line, rev=False) -> int:
    for i in range(len(line) + 1):
        for _, word in enumerate(["1", "2", "3", "4", "5", "6", "7", "8", "9", 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine']):
            if rev:
                if word in line[(len(line) -  i):]:
                    return str_values[word]
            else:
                if word in line[:i]:
                    return str_values[word]

    return 0


with open ("./input.txt", "r") as f:
    for line in f.read().splitlines():
        prev = sum_of_calibration_values
        sum_of_calibration_values += (get_first_int(line) * 10)
        sum_of_calibration_values += get_first_int(line, True)
        print(sum_of_calibration_values - prev)



print(sum_of_calibration_values)













