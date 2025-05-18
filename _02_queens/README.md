# Problem Formulation

## Used Thecnique
### Depth First Search (DFS)
![alt text](images/deque.png)

## Initial States
```python
# Unsolvable
[
    [4, 6, 2],
    [8, 1, 3],
    [7, 5, 0]
] 
# Solvable
[
    [6, 4, 2],
    [8, 1, 3],
    [7, 5, 0]
]
# Solvable
[
    [1, 2  3],
    [4, 5, 6],
    [7, 8, 0]
]
# Solvable
[
    [7, 5, 4],
    [1, 0, 3],
    [2, 6, 8]
]
```

## Objective State
- Any configuration of the 8x8 board which leads to no queens attacking each other.

## Objective Test
![alt text](images/test_objective.png)

## Set of Operations

## Successor Function

### Generating Children
![alt text](images/directions_application.png)

## Results
![unsolvable](images/unsolvable.png)
![solvable_01](images/solvable_01.png)
![solvable_02](images/solvable_02.png)
![solvable_03](images/solvable_03.png)