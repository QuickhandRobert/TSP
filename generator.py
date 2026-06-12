import json
import random
import math

random.seed(42)

N = 200

cities = []

for _ in range(N):
    x = random.randint(0, 10000)
    y = random.randint(0, 10000)
    cities.append((x, y))

matrix = [[0] * N for _ in range(N)]

for i in range(N):
    for j in range(i + 1, N):

        x1, y1 = cities[i]
        x2, y2 = cities[j]

        d = round(math.hypot(x1 - x2, y1 - y2))

        matrix[i][j] = d
        matrix[j][i] = d

with open("file.json", "w") as f:
    json.dump(
        {
            "n": N,
            "matrix": matrix
        },
        f
    )

print("generated")