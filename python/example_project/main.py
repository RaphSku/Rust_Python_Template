import time
import numpy as np
import example_project


def main():
    array = [i for i in range(1, 10001)]
    print(len(array))

    start = time.perf_counter()
    print(example_project.parallelized_sum_values_in_array(array))
    end = time.perf_counter()
    print("Time parallelized:", end - start)

    start = time.perf_counter()
    print(example_project.sum_values_in_array(array))
    end = time.perf_counter()
    print("Time:", end - start)

    start = time.perf_counter()
    print(np.sum(array))
    end = time.perf_counter()
    print("Numpy Python Time:", end - start)

    start = time.perf_counter()
    result = 0
    for item in array:
        result += item
    print(result)
    end = time.perf_counter()
    print("Python Time:", end - start)


if __name__ == "__main__":
    main()