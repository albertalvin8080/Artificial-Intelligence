# Problem Formulation

- Get the shortest path in minutes between two stations (e.g. E1 to E14) including change of lines.

## Used Thecnique
### A*
![alt text](images/heuristic.png)
![alt text](images/g.png)
![alt text](images/node_creation.png)

## Initial States
- Any of the stations from E1 to E14.

![alt text](images/distances.png)
![alt text](images/real_distances.png)

## Objective State
- The destination station (e.g., E14) someone tends to arrive.

## Objective Test
- Check if the current station matches the destination station.
  
![alt text](images/test_objective.png)

## Set of Operations
- Move to a directly connected station.
- Change lines if necessary at interchange stations (+5 minutes of walk).

## Successor Function
- For a given station, return all directly connected stations with their respective travel times and the line used.

### Generating Children
- Each child represents a neighboring station reachable from the current station.
- For each child, the algorithm calculates:
  - The cumulative travel time (cost so far).
  - The estimated remaining time using the heuristic.
  
![alt text](images/generating_children.png)

## Results
![1](images/matrix.png)
![1](images/e14_to_e3.png)
![1](images/e1_to_e6.png)
![1](images/e5_to_e9.png)