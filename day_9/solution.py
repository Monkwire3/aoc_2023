class Sequence:
    def __init__(self, history: list[int]):
        self.history = history


    @classmethod
    def get_diffs(cls, seq):

        diffs = []

        for i in range(1, len(seq)):
            diffs.append(seq[i -1] - seq[i])

        return diffs


    def find_next_step(self) -> int:

        next_step = self.history[0]

        diffs = Sequence.get_diffs(self.history)

        while any(diffs):
            next_step += diffs[0]
            diffs = Sequence.get_diffs(diffs)

        self.next_step = next_step
        return next_step



def main():
    step_sum = 0
    with open("./input.txt") as f:
        for line in f.read().splitlines():
            sequence = Sequence(list(map(lambda x: int(x), line.split(" "))))
            step_sum += sequence.find_next_step()


            print(sequence.history, sequence.next_step)

    print(step_sum)





main()









