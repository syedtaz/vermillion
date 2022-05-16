import sys
import pandas as pd
from matplotlib import pyplot as plt

def main():
    file = sys.argv.pop();
    df = pd.read_csv(file)
    df.plot(x=df.columns[0], y=df.columns[1:])
    plt.ylabel("Molecule Count")
    plt.savefig("data/plot.png")

if __name__ == "__main__":
    main()