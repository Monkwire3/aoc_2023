
# 1 m/s held - speed + one m/s



class Race:
    def __init__(self, time, best_distance):
        self.time = time
        self.best_distance = best_distance


    def get_distance(self, time_held):
        return time_held * (self.time - time_held)

    def get_all_distances(self):

        distances = []

        for t in range(self.time):
            print(f"{t}/{self.time}")
            distances.append(self.get_distance(t))

        return distances

    def get_ways_to_beat_record(self):
        return len(list(filter(lambda x: x > self.best_distance, self.get_all_distances())))



def main():
    times = 44899691
    dists = 277113618901768




    margin = Race(times, dists).get_ways_to_beat_record()

    print(margin)




main()





