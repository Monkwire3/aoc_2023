from os import sched_get_priority_max


class RangeMap:
    def __init__(self, map_name: str):
        name_parts = map_name.split("-")
        self.name = name_parts[0]
        self.end_type = name_parts[2]
        self.mappings = dict()

    def add_range(self, range_string: str):
        range_parts = map(lambda x: int(x), range_string.split(" "))
        range_start, range_end, range_length = range_parts

        dist = range_end - range_start

        if range_start < range_end:
            self.mappings[range_start] = dist
        else:
            self.mappings[range_start - dist] = dist

        # for i in range(range_length):
        #     print('adding mapping from ', range_start + i, 'to', range_start + dist + i)
        #     self.mappings[range_start + i] = range_start + dist + i

    def convert(self, start: int):
        prev_range = 0
        for r in self.mappings:
            if start > r:
                prev_range = r
            else:
                break

        return self.mappings[prev_range] + start



class Seed():
    def __init__(self, seed_type: str, seed_value: int):
        self.seed_type = seed_type
        self.seed_value = seed_value
        self.stats = {seed_type : seed_value}

    def transition_seed(self, r_map: RangeMap):
        self.seed_type = r_map.end_type
        self.seed_value = r_map.convert(self.seed_value)
        self.stats[self.seed_type] = self.seed_value




class SeedManager():
    def __init__(self):
        self.range_maps = dict()
        self.seeds = list()


    def add_map(self, map_name: str, ranges: list[str]):
        range_map = RangeMap(map_name)

        for r in ranges:
            range_map.add_range(r)


        self.range_maps[range_map.name] = range_map

    def add_seeds(self, seeds: list[str]):
        for s in seeds:
            self.seeds.append(Seed('seed', int(s)))

    def get_locations(self):
        locations = []
        all_seeds_at_locations = False


        while not all_seeds_at_locations:
            all_seeds_at_locations = True
            for seed in self.seeds:
                if seed.seed_type != "location":
                    all_seeds_at_locations = False
                seed.transition_seed(r_map=self.range_maps[seed.seed_type])

        for seed in self.seeds:
            locations.append(seed.seed_value)


        return locations








def parse_input(path: str) -> SeedManager:

    seed_manager = SeedManager()
    current_map = ""
    ranges = []

    with open(path, "r") as f:
        for line in f.read().splitlines():
            if not len(line):
                if current_map:
                    seed_manager.add_map(current_map, ranges)


                current_map = ""
                ranges = []
                continue


            elif not len(current_map):
                if line[5] == ":":
                    seeds = line.split(": ")[1].split(" ")
                    seed_manager.add_seeds(seeds=seeds)

                else:
                    current_map = line.split(" ")[0]
            else:
                ranges.append(line.strip())

    return seed_manager


def main():
    seed_manager = parse_input("./input.txt")

    for rm in seed_manager.range_maps:
        print(rm)


    print(seed_manager.get_locations())


main()

