import pandas as pd
import numpy as np
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import StandardScaler, LabelEncoder
from sklearn.neural_network import MLPClassifier
from sklearn.metrics import accuracy_score

df = pd.read_csv(r"C:\Users\a.bandeira.sobral\Documents\_git_repositories\Artificial-Inteligence\_02\_01_knn\breast_cancer_wisconsin.csv")

X = df.drop(columns=["id", "diagnosis", "Unnamed: 32"]).values
y = LabelEncoder().fit_transform(df["diagnosis"])  # Maligno(M)=1, Benigno(B)=0

# Normalizar dados
scaler = StandardScaler()
X = scaler.fit_transform(X)

X_train, X_test, y_train, y_test = train_test_split(
    X, y, test_size=0.3, random_state=42
)

# Configurações (15 redes)
configs = [
    # ---- 1 camada escondida ----
    {"hidden_layer_sizes": (10,), "activation": "relu"},
    {"hidden_layer_sizes": (20,), "activation": "relu"},
    {"hidden_layer_sizes": (50,), "activation": "relu"},
    {"hidden_layer_sizes": (10,), "activation": "tanh"},
    {"hidden_layer_sizes": (20,), "activation": "tanh"},
    {"hidden_layer_sizes": (50,), "activation": "tanh"},
    {"hidden_layer_sizes": (10,), "activation": "logistic"},
    {"hidden_layer_sizes": (20,), "activation": "logistic"},
    {"hidden_layer_sizes": (50,), "activation": "logistic"},
    
    # ---- 2 camadas escondidas ----
    {"hidden_layer_sizes": (50, 20), "activation": "relu"},
    {"hidden_layer_sizes": (100, 50), "activation": "relu"},
    {"hidden_layer_sizes": (50, 20), "activation": "tanh"},
    {"hidden_layer_sizes": (100, 50), "activation": "tanh"},
    {"hidden_layer_sizes": (50, 20), "activation": "logistic"},
    {"hidden_layer_sizes": (100, 50), "activation": "logistic"},
]

# Treinar as redes
results = []
for i, cfg in enumerate(configs, start=1):
    mlp = MLPClassifier(
        hidden_layer_sizes=cfg["hidden_layer_sizes"],
        activation=cfg["activation"],
        solver="adam",
        max_iter=5000,
        random_state=42
    )
    mlp.fit(X_train, y_train)
    y_pred = mlp.predict(X_test)
    acc = accuracy_score(y_test, y_pred)
    results.append((i, cfg["hidden_layer_sizes"], cfg["activation"], acc))

# resultados
print(f"{'Rede':<7} | {'Camadas':<15} | {'Ativação':<10} | {'Acurácia':<8}")
print("-"*55)
for r in results:
    print(f"{'Rede '+str(r[0]):<7} | {str(r[1]):<15} | {r[2]:<10} | {r[3]:<8.4f}")

# results[3] é a acuracia
max_acc = max(results, key=lambda x: x[3])[3]
# filtrar todas as redes com essa acurácia
best_configs = [r for r in results if r[3] == max_acc]

print("\nMelhor(s) configuração(ões):")
for best in best_configs:
    print(f"Rede {best[0]} | Camadas: {best[1]} | Ativação: {best[2]} | Acurácia: {best[3]:.4f}")