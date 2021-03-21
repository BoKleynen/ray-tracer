import matplotlib.pyplot as plt


with open("output.txt", "r") as file:
    counts = list(map(int, file.read().split(",")))
    counts = [counts[i:i+1920] for i in range(0, len(counts), 1920)]
    fig = plt.imshow(counts, origin="lower")
    plt.colorbar(fig)
    fig.axes.get_xaxis().set_visible(False)
    fig.axes.get_yaxis().set_visible(False)
    plt.savefig("test.png")
