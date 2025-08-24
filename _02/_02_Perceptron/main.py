import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from sklearn.model_selection import train_test_split

data = pd.read_csv(
    r'C:\Users\a.bandeira.sobral\Documents\_git_repositories\Artificial-Inteligence\_02\_02_Perceptron\perceptron.txt', 
    sep=';', 
    header=None
)
X = data.iloc[:, :2].to_numpy()
y = data.iloc[:, 2].to_numpy()

# Labels: 1 -> 0 e 2 -> 1
y = np.where(y == 1, 0, 1)

X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.3, random_state=42)

# bias (w0 = 1)
def add_bias(X):
    return np.insert(X, 0, 1, axis=1)

# '1' as the first feature in every input vector
X_train_bias = add_bias(X_train)
X_test_bias = add_bias(X_test)

class Perceptron:
    def __init__(self, lr=0.01, n_iter=1, initial_weights=None):
        print(f"Initial LR: {lr}")
        print(f"Total Iterations: {n_iter}")
        self.lr = lr
        self.n_iter = n_iter
        self.initial_weights = initial_weights
        self.weights = None
        
    def activation(self, z):
        return np.where(z >= 0, 1, 0)

    def fit(self, X, y):
        if self.initial_weights is not None:
            self.weights = self.initial_weights
        else:
            self.weights = np.zeros(X.shape[1])
        print(f"Initial weights: {self.weights}")
        for _ in range(self.n_iter):
            for xi, target in zip(X, y):
                z = np.dot(xi, self.weights)
                y_pred = self.activation(z)
                error = target - y_pred
                self.weights += self.lr * error * xi

    def predict(self, X):
        return self.activation(np.dot(X, self.weights))


def plot_decision_boundary(X, y, model):
        # Define the range for the x-axis (feature 1), with a margin of -1 and +1
        x_min, x_max = X[:,1].min() - 1, X[:,1].max() + 1
        
        # Define the range for the y-axis (feature 2), with a margin of -1 and +1
        y_min, y_max = X[:,2].min() - 1, X[:,2].max() + 1
        
        # Create a grid of points covering the feature space (100 steps in each axis)
        xx, yy = np.meshgrid(np.linspace(x_min, x_max, 100),
                             np.linspace(y_min, y_max, 100))
        
        # Flatten the grid and add a column of ones for the bias term
        # This results in a matrix of shape (number_of_points, 3) -> [bias, x, y]
        grid = np.c_[np.ones(xx.ravel().shape), xx.ravel(), yy.ravel()]
        
        # Use the trained model to predict labels for every point in the grid
        preds = model.predict(grid).reshape(xx.shape)
        
        # Plot the decision boundary by coloring regions based on predicted class
        plt.contourf(xx, yy, preds, alpha=0.3, cmap='bwr')
        
        # Plot the actual data points, coloring them by their true labels
        plt.scatter(X[:,1], X[:,2], c=y, cmap='bwr', edgecolors='k')
        plt.title("Perceptron - Fronteira de Decisão")                                      
        plt.xlabel("Característica 1")
        plt.ylabel("Característica 2")
        plt.grid(True)
        plt.show()
        

if __name__ == "__main__":
    perceptron = Perceptron(lr=0.01, n_iter=2)
    perceptron.fit(X_train_bias, y_train)
    
    y_pred = perceptron.predict(X_test_bias)
    accuracy = np.mean(y_pred == y_test)
    print("Acurácia na base de teste:", round(accuracy * 100, 2), "%")

    plot_decision_boundary(X_test_bias, y_test, perceptron)
