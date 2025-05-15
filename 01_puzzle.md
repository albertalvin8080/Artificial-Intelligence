# Problem Formulation

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
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8]
]
# Solvable
[
    [7, 5, 4],
    [1, 0, 3],
    [2, 6, 8]
]
```

## Objective State
```python
[
    [1, 2  3],
    [4, 5, 6],
    [7, 8, 0]
]
```

## Set of Operations
- Move the "empty tile" on the 3x3 board

## Successor Function
### Matrix used to generate children
```python
[(-1, 0), (1, 0), (0, -1), (0, 1)]
```
- (-1, 0) -> Retornar uma linha
- (0, -1) -> Retornar uma coluna
- (+1, 0) -> Avançar uma linha
- (0, +1) -> Avançar uma coluna

## Resultados
![insolvable](images/01_puzzle/unsolvable.png)
![solvable_01](images/01_puzzle/solvable_01.png)
![solvable_02](images/01_puzzle/solvable_02.png)
![solvable_03](images/01_puzzle/solvable_03.png)