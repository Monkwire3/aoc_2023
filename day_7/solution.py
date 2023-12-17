from __future__ import annotations
from collections import Counter, defaultdict
from enum import IntEnum
from functools import reduce

class HandType(IntEnum):
    HighCard = 0
    OnePair = 1
    TwoPair = 2
    ThreeKind = 3
    FullHouse = 4
    FourKind = 5
    FiveKind = 6

class Hand:
    def __init__(self, cards: str, bid: int):
        self.cards = cards
        self.bid = bid
        self.rank = 0

        counts = Counter(self.cards).most_common()
        most_freq = counts[0][0]

        while most_freq == "J" and len(counts):
            most_freq = counts.pop(0)[0]


        cards_without_joker = self.cards.replace("J", most_freq)

        match len(set(cards_without_joker)):
            case 5:
                self.hand_type = HandType.HighCard
            case 4:
                self.hand_type = HandType.OnePair
            case 3:
                self.hand_type = HandType.ThreeKind if max(Counter(cards_without_joker).values()) == 3 else HandType.TwoPair
            case 2:
                self.hand_type = HandType.FourKind if max(Counter(cards_without_joker).values()) == 4 else HandType.FullHouse
            case 1:
                self.hand_type = HandType.FiveKind
            case _:
                raise AssertionError

    def key(self) -> tuple[HandType, list[int]]:
        card_values = {card: i for i, card in enumerate("J23456789TQKA")}

        return (self.hand_type, [card_values[card] for card in self.cards])

    def __str__(self) -> str:
        return f"{self.cards} {self.hand_type} {self.rank}"


class Game:
    def __init__(self, hands: list[Hand]):
        self.hands = hands
        self._sort()
        self._rank()

    @classmethod
    def build_from_file(cls, path: str) -> Game:
        hands = []
        with open(path, "r") as f:
            for line in f.read().splitlines():
                cards, bid = line.split(" ")

                hands.append(Hand(cards=cards, bid=int(bid)))

        return Game(hands=hands)

    def _sort(self):
        self.hands.sort(key=Hand.key)

    def _rank(self):
        for i, hand in enumerate(self.hands):
            hand.rank = i + 1

    def total_winnings(self):
        return reduce(lambda acc, hand: acc + (hand.bid * hand.rank), self.hands, 0)


def main():
    game = Game.build_from_file("./input.txt")
    for h in game.hands:
        print(h)
    print(game.total_winnings())


main()
