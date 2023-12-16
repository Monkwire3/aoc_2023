from collections import deque, defaultdict



class Card:
    def __init__(self, number: int, winning_numbers: list[str], your_numbers: list[str]):
        self.number = number
        self.winning_numbers = winning_numbers
        self.your_numbers = your_numbers
        self.value = 0
        self.copies = []


        for n in self.your_numbers:
            if n  != "" and n in self.winning_numbers:
                if not self.value:
                    self.value = 1
                else:
                    self.value *= 2
                self.copies.append(self.number + 1 + len(self.copies))




class CardList:
    def __init__(self, path: str):
        self.cards = defaultdict(lambda: Card(0, [], []))
        self.value = 0
        self.queue = deque()

        with open(path, "r") as f:
            for line in f.read().splitlines():
                all_data = line.split(":")[1].split("|")
                card_no = int(line.split(":")[0].split(" ")[-1])
                winning_numbers = all_data[0].strip().split(" ")
                your_numbers = all_data[1].strip().split(" ")
                self.cards[card_no] = Card(card_no, winning_numbers, your_numbers)
                self.queue.append(card_no)

    def get_total_value(self) -> int:
        while self.queue:
            current = self.cards[self.queue.popleft()]
            self.value += 1
            self.queue.extend(current.copies)

        return self.value




def main():
    card_list = CardList("./input.txt")
    print(card_list.get_total_value())

    card_list = CardList("./test_input.txt")
    print(card_list.get_total_value())

main()





