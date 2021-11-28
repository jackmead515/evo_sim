import math
import random

class Neuron:

    def __init__(self, weights, bias):
        self.weights = weights
        self.bias = bias

    def compute(self, inputs):
        total = 0
        for i in range(len(inputs)):
            total += self.weights[i] * inputs[i]
        print(f'total before bias: {total}')
        total += self.bias
        print(f'total after bias: {total}')
        total = 1 / (1 + math.exp(-total))
        return total


class Layer:

    def __init__(self, neurons):
        self.neurons = neurons

    def compute(self, inputs):
        outputs = []
        for i in range(len(inputs)):
            outputs.append(self.neurons[i].compute(inputs))
        return outputs


def random_neuron(size) -> Neuron:
    weights = [random.random() for _ in range(size)]
    bias = random.random()
    return Neuron(weights, bias)


if __name__ == "__main__":

    layer_size = 5
    inputs = [0, 0, 0, 0]

    neurons = [random_neuron(len(inputs)) for _ in range(layer_size)]
    layer = Layer(neurons)

    outputs = layer.compute(inputs)

    print(outputs)

