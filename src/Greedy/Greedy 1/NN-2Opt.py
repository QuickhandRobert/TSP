from time import perf_counter


def tour_cost(tour, dist):
    n = len(tour)
    cost = 0

    for i in range(n - 1):
        cost += dist[tour[i]][tour[i + 1]]

    cost += dist[tour[-1]][tour[0]]

    return cost


def nearest_neighbor(dist, start=0):
    n = len(dist)

    visited = [False] * n
    visited[start] = True

    tour = [start]
    current = start

    for _ in range(n - 1):
        next_city = None
        best_distance = float("inf")

        for city in range(n):
            if not visited[city] and dist[current][city] < best_distance:
                best_distance = dist[current][city]
                next_city = city

        tour.append(next_city)
        visited[next_city] = True
        current = next_city

    return tour


def two_opt(tour, dist):

    n = len(tour)
    improved = True

    while improved:
        improved = False

        for i in range(1, n - 1):
            for j in range(i + 1, n):
                a = tour[i - 1]
                b = tour[i]

                c = tour[j]
                d = tour[(j + 1) % n]

                old_cost = dist[a][b] + dist[c][d]
                new_cost = dist[a][c] + dist[b][d]

                if new_cost < old_cost:
                    tour[i : j + 1] = reversed(tour[i : j + 1])

                    improved = True

    return tour


# Input -----------------------------


def load_from_json(filename):
    import json

    with open(filename) as f:
        data = json.load(f)

    return data["adj_mat"], data["parameters"][1][0]


dist, lower_bound = load_from_json(input("Enter filename: "))


# Nearest Neighbor ------------------


start_time = perf_counter()

nn_tour = nearest_neighbor(dist)

nn_time = (perf_counter() - start_time) * 1000

nn_cost = tour_cost(nn_tour, dist)

# NN + 2OPT -------------------------

start_time = perf_counter()

improved_tour = two_opt(nn_tour.copy(), dist)

opt_time = (perf_counter() - start_time) * 1000

improved_cost = tour_cost(improved_tour, dist)

# Output ----------------------------

print("Nearest Neighbor")
print("Cost:", nn_cost)
print("Ratio to Lower Bound:", nn_cost / lower_bound)
print("Tour:", nn_tour + [nn_tour[0]])
print("Time(ms):", round(nn_time, 3))

print()

print("Nearest Neighbor + 2-Opt")
print("Cost:", improved_cost)
print("Ratio to Lower Bound:", improved_cost / lower_bound)
print("Tour:", improved_tour + [improved_tour[0]])
print("Time(ms):", round(opt_time, 3))
