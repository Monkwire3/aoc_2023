import math
class Traveler:
    def __init__(self, start_node, graph, directions):
        self.start_node = start_node
        self.current_node = start_node
        self.graph = graph
        self.directions = directions
        self.visited = set()


    def advance_one_step(self, direction):
        self.current_node = self.graph[self.current_node][direction]

    def on_end_node(self):
        return self.current_node[-1] == "Z"

    def print_info(self):
        print('Traveler: ', self.current_node)


    def get_steps_to_destination(self):
        self.visited = set()
        steps_to_destination = 0
        steps_to_repeat = 0
        steps = 0
        dir_idx = 0
        while True:
            if self.current_node[-1] == "Z":
                steps_to_destination = steps

            step_signature = (f"{self.current_node}_{dir_idx % len(self.directions)}")
            if step_signature in self.visited:
                steps_to_repeat = steps
                break
            self.visited.add(step_signature)


            self.advance_one_step(self.directions[dir_idx % len(self.directions)])

            steps += 1
            dir_idx += 1



        return steps_to_destination


class TravelerManager:
    def __init__(self, graph, directions):
        self.travelers = []
        self.steps = 0
        self.graph = graph
        self.directions = directions

        for key in graph:
            if key[-1] == "A":
                self._create_traveler(key)

    def _create_traveler(self, start_node):
        t = Traveler(start_node=start_node, graph=self.graph, directions=self.directions)
        self.travelers.append(t)

    def advance_one_step(self, direction):
        for t in self.travelers:
            t.advance_one_step(direction)
            self.steps += 1


    def count_t_at_destination(self):
        tad = 0
        for t in self.travelers:
            if t.current_node[-1] == "Z":
                tad += 1

        if tad > 2:
            print(f"{self.steps} {tad}/{len(self.travelers)}")


    def get_steps_to_destination(self):
        res = []
        for t in self.travelers:
            res.append(t.get_steps_to_destination())
            print(f"travelers solved: {len(res)}/{len(self.travelers)}")


        print('lcm ', math.lcm(*res))





        print(res)
        return res


    def all_travelers_at_destination(self):
        for t in self.travelers:
            if t.current_node[-1] != "Z":
                return False
        return True

    def print_all(self):
        locations = []
        for t in self.travelers:
            locations.append(t.current_node)

        print(self.steps, locations)






def build_graph(raw_input):
    node_map = {}

    with open(raw_input, "r") as f:
        for line in f.read().splitlines():
            node_map[line[0:3]] = {"L": line[7:10], "R": line[12:15]}

    return node_map


def solve(map_path, directions):
    node_map = build_graph(map_path)
    traveler_manager = TravelerManager(node_map, directions=directions)


    print("to destination: ")
    print(traveler_manager.get_steps_to_destination())


    # while not traveler_manager.all_travelers_at_destination():
    #     traveler_manager.advance_one_step(directions[dir_idx % len(directions)])
    #     traveler_manager.count_t_at_destination()
    #     # traveler_manager.print_all()





def main():
    main_directions = "LLRRLRRRLLRLRRLLLLRLRRLRRRLRLRRRLLRRLRRRLLRRLRRLRRLLRRRLRRLRRLRRRLRRLRLRLRRLRRLRRRLLRRLLLRRLRRRLRRRLRRRLRRLRRRLRLLRLRRRLRLRRLLRLRRRLRRRLRLRRRLRRRLRLRLRRLRRLRLRRLLRRRLRRRLRRRLLRRRLRLRLRLRLLRRRLRRRLRRLRRRLLRLRRLRRLRRRLRRRLRRLRLRLRRRLRRLRRLRRRLLRRLRLRLRRRLRLRLRRLRRLLRRLRRRLLRLLRLRLRRRR" * 100

    print(solve("./input.txt", main_directions))



main()
