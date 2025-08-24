import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from sklearn.cluster import KMeans
from sklearn.metrics import adjusted_rand_score

data = pd.read_csv(
    r'C:\Users\a.bandeira.sobral\Documents\_git_repositories\Artificial-Inteligence\_02\_04_kMeans\kmeans.txt',
    sep=';', 
    header=None
)

X = data.iloc[:, :2].to_numpy()
y_true = data.iloc[:, 2].to_numpy()

# Variar entre k = 2 e k = 5
for k in [2, 3, 4, 5]:
    # n_init runs the algorithm multiple times with different random initializations and then keeps the best solution
    kmeans = KMeans(n_clusters=k, random_state=42, n_init=10)
    y_kmeans = kmeans.fit_predict(X)

    # Visualização dos clusters encontrados
    plt.figure(figsize=(7, 5))
    plt.scatter(X[:, 0], X[:, 1], c=y_kmeans, cmap='viridis', s=50, alpha=0.7, edgecolors='k')
    plt.scatter(kmeans.cluster_centers_[:, 0], kmeans.cluster_centers_[:, 1], 
                c='red', s=200, marker='X', label='Centroides')
    plt.title(f"K-Means - Agrupamento com k={k}")
    plt.xlabel("Característica 1")
    plt.ylabel("Característica 2")
    plt.legend()
    plt.grid(True)
    plt.show()

    # Comparação com rótulos reais
    score = adjusted_rand_score(y_true, y_kmeans)
    print(f"Adjusted Rand Index (k={k}):", round(score, 3))
