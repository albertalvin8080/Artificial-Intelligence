import pandas as pd
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import StandardScaler
from sklearn.neighbors import KNeighborsClassifier
from sklearn.metrics import accuracy_score, classification_report, confusion_matrix

df = pd.read_csv(r"C:\Users\a.bandeira.sobral\Documents\_git_repositories\Artificial-Inteligence\_02\_01_knn\breast_cancer_wisconsin.csv")
print(df.shape)
print(df.head())

df = df.drop(columns=['id'], errors='ignore')

# B = 0 (benign), M = 1 (malignant)
df['diagnosis'] = df['diagnosis'].map({'B': 0, 'M': 1})

# drop columns with all NaN values
df = df.dropna(axis=1, how='all') 

X = df.drop(columns=['diagnosis']).to_numpy()
y = df['diagnosis'].to_numpy()

# normalization
scaler = StandardScaler()
X_scaled = scaler.fit_transform(X)

X_train, X_test, y_train, y_test = train_test_split(
    X_scaled, y, test_size=0.3, stratify=y, random_state=42
)

def run_knn(p_value, metric_name, print_matrix=False):
    print(f"\n--- Distance: {metric_name} (p={p_value}) ---")
    for k in [1, 3, 5, 7, 9, 11]:
        knn = KNeighborsClassifier(n_neighbors=k, metric=metric_name, p=p_value)
        knn.fit(X_train, y_train)
        y_pred = knn.predict(X_test)
        acc = accuracy_score(y_test, y_pred)
        print(f"k = {k:2d} -> Accuracy: {acc*100:.2f}%")
        if print_matrix:
            print(classification_report(y_test, y_pred, target_names=['Benign','Malignant']))
            print("Confusion Matrix:")
            print(confusion_matrix(y_test, y_pred))


if __name__ == "__main__":
    run_knn(p_value=1, metric_name='manhattan', print_matrix=False)
    run_knn(p_value=2, metric_name='euclidean', print_matrix=False)
    run_knn(p_value=3, metric_name='minkowski', print_matrix=False)
