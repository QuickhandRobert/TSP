# The Travelling Salesman Problem

The travelling salesman problem is one of the most famous/infamous problems in history of computer science. The problem goes as follows (Source: Wikipedia):

In the theory of computational complexity, the travelling salesman problem (TSP) asks the following question: "Given a list of cities and the distances between each pair of cities, what is the shortest possible route that visits each city exactly once and returns to the origin city?"

The problem is best known for its NP-Hardness, therefore being hard to solve in an asymptotically acceptable time. Despite having several applications in many different fields, some not even related to computer science.

In this repository, we will take a look at 3 separate algorithms that either solve or provide a "good enough" approximation of the actual solution. The first algorithm utilizes the technique of dynamic programming, and finds the actual solution to the problem. The second two will follow a greedy approach, trying to approximate the best possible answer while not taking an eternity to do so.

-------------------------------------------

## Benchmarking each algorithm

Benchmarking is done by testing two factors: 

**1. The execution time:**

We shall test each algorithm for how low they take to complete. This is either done by the time utility of your favorite POSIX shell, or by the code itself.

**2. How good the solution is:**

The dynamic programming algorithm is obviously excluded from this benchmark. Since it always gives the optimal solution and benchmarking that would be pointless. 

One good benchmark we could utilize goes like so:

$x + y$
$Approximates_Solution \div Lower_Bound$
With a lower bound given by:


-------------------------------------------

## Approach 1: Dynamic Programming
