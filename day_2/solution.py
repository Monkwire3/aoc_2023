def get_minimum_set(r):
    cubes = r.strip(" ").split(" ")

    max_seen = {
            'red' : 0,
            'green' : 0,
            'blue' : 0
            }

    for i, cube in enumerate(cubes):
        if len(cube) and i % 2 == 0:
            qty = int(cube)
            color = cubes[i + 1] if cubes[i + 1][-1] != "," else cubes[i + 1][:-1]
            if qty > max_seen[color]:
                max_seen[color] = qty


    print(max_seen)
    return max_seen


def get_game_power(g):
    max_seen = {
            'red': 0,
            'green': 0,
            'blue': 0
            }



    for r in g:
        round_cubes = get_minimum_set(r)

        for k in round_cubes:
            if round_cubes[k] > max_seen[k]:
                max_seen[k] = round_cubes[k]
    game_power =  max_seen['red'] * max_seen['green'] * max_seen['blue']
    print("===========")
    print(max_seen)
    print('game power: ', game_power)
    print("=======")
    return game_power



game_power_sum = 0
with open("./input.txt", "r") as f:
    for line in f.read().splitlines():
        idx_end = line.find(":")
        game_id = line[5:idx_end]
        rounds = line[idx_end + 2:].strip().split(";")
        game_power_sum += get_game_power(rounds)



print(game_power_sum)

