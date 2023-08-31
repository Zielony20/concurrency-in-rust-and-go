import subprocess
import time
import matplotlib.pyplot as plt
from tqdm import tqdm
from datetime import datetime
from datetime import datetime
import pandas as pd
import numpy as np
import psutil

def compile_tests(programs):
    for program in programs:
        subprocess.call(program, shell=True)

def measure_execution_time(command):
    start_time = time.time()
    subprocess.call(command, shell=True)
    end_time = time.time()
    execution_time = end_time - start_time
    return execution_time


def run_programs_with_parameters(programs, parameters, parameters2):
    results = {}
    results.clear()
    for program in programs:
        results[program] = []
        for param1, param2 in tqdm(zip(parameters, parameters2), total=len(parameters)):
            command = f"{program} {param1} {param2}"  
            execution_time = measure_execution_time(command)
            results[program].append(execution_time)
    return results

def plot_results(results, parameters, program_names, x_labels, y_labels, postfix):
    plt.clf()
    i = 0
    for program, times in results.items():
        plt.plot(parameters, times, label=program_names[i])
        i += 1
    
    plt.xlabel(x_labels)
    plt.ylabel(y_labels)
    plt.legend()

    # Zapisywanie wykresu do pliku o nazwie zgodnej z dzisiejszą datą i godziną
    current_datetime = datetime.now().strftime("%Y-%m-%d_%H-%M-%S")
    plot_filename = f"wykres_{current_datetime}_{postfix}.png"
    plt.savefig(plot_filename)

    # Zapisywanie danych do pliku CSV
    df = pd.DataFrame(results, index=parameters)
    csv_filename = f"dane_{current_datetime}_{postfix}.csv"
    df.to_csv(csv_filename)

def matrix_tests(threads=1):
    parameters = [threads for x in range(1, 2000)]
    parameters2 = [x for x in range(1, 2000)]
    compile_tests(['cargo build --release --manifest-path=../Rust_benchmark/matrix/Cargo.toml','cd ../Go_benchmark && go build && cd ../PlatformForBenchmarks'])
    programs = ["../Go_benchmark/matrix ","../Rust_benchmark/matrix/target/release/matrix "]
    names = ["Go","Rust"]
    
    results = run_programs_with_parameters(programs, parameters, parameters2)
    plot_results(results, parameters2, names, 'Rozmiar macierzy', 'Czas trwania (s)',"matrix_tests")
    results.clear()

def sorting_tests3():
    parameters = [x for x in range(500, 2001, 500)]
    parameters2 = [20 for x in range(1, 2001, 10)]
    compile_tests(['cargo build --release --manifest-path=../Rust_distributed_sorting/distributed_sorting/Cargo.toml','cd ../Go_distributed_sorting && go build && cd ../PlatformForBenchmarks'])
    programs = ["../Rust_distributed_sorting/distributed_sorting/target/release/distributed_sorting"]
    
    names = ["Rust"]
    
    results = run_programs_with_parameters(programs, parameters, parameters2)
    plot_results(results, parameters, names, 'Liczba wątków', 'Czas trwania (s)',"sorting3")
    results.clear()
    #print(run_programs_and_return_stats(programs, 160, 20))


def sorting_tests2():
    parameters = [16 for x in range(1, 20001, 100)]
    parameters2 = [x for x in range(1, 20001, 100)]
    compile_tests(['cargo build --release --manifest-path=../Rust_distributed_sorting/distributed_sorting/Cargo.toml','cd ../Go_distributed_sorting && go build && cd ../PlatformForBenchmarks'])
    programs = ["../Go_distributed_sorting/distributed_sorting","../Rust_distributed_sorting/distributed_sorting/target/release/distributed_sorting"]
    
    names = ["Go","Rust"]
    
    results = run_programs_with_parameters(programs, parameters, parameters2)
    plot_results(results, parameters2, names, 'Rozmiar tablicy na wątek', 'Czas trwania (s)',"sorting_tests_16wątków (buffor delete)")
    results.clear()
    #print(run_programs_and_return_stats(programs, 160, 20))

def sorting_tests():
    parameters = [x for x in range(64, 512, 16)]
    parameters2 = [10000 for x in range(64, 512, 16)]
    compile_tests(['cargo build --release --manifest-path=../Rust_distributed_sorting/distributed_sorting/Cargo.toml','cd ../Go_distributed_sorting && go build && cd ../PlatformForBenchmarks'])
    programs = ["../Go_distributed_sorting/distributed_sorting","../Rust_distributed_sorting/distributed_sorting/target/release/distributed_sorting"]
    
    names = ["Go","Rust"]
    
    results = run_programs_with_parameters(programs, parameters, parameters2)
    plot_results(results, parameters, names, 'Liczba wątków', 'Czas trwania (s)',"sorting_tests 10k per thread (buffor delete)")
    results.clear()

def mutex_tests():
    parameters = [x for x in range(2, 10001, 1)]
    parameters2 = [1000 for x in range(2, 10001, 1)]
    compile_tests(['cargo build --release --manifest-path=../Rust_mutex/mutex/Cargo.toml','cd ../Go_mutex && go build && cd ../PlatformForBenchmarks'])
    programs = ["../Go_mutex/mutex","../Rust_mutex/mutex/target/release/mutex"]
    
    names = ["Go","Rust"]
    
    results = run_programs_with_parameters(programs, parameters, parameters2)
    plot_results(results, parameters, names, 'Liczba wątków', 'Czas trwania (s)',"mutex_tests_tysiac_kazdy_watek")
    results.clear()

def mutex_tests2():
    parameters = [16 for x in range(2, 100001, 1)]
    parameters2 = [x for x in range(2, 100001, 1)]
    compile_tests(['cargo build --release --manifest-path=../Rust_mutex/mutex/Cargo.toml','cd ../Go_mutex && go build && cd ../PlatformForBenchmarks'])
    programs = ["../Go_mutex/mutex","../Rust_mutex/mutex/target/release/mutex"]
    
    names = ["Go","Rust"]
    
    results = run_programs_with_parameters(programs, parameters, parameters2)
    plot_results(results, parameters2, names, 'Liczba wiadomości na każdy wątek', 'Czas trwania (s)',"mutex_tests_16wątków_100k")
    results.clear()
    #print("Mutex:" ,run_programs_and_return_stats(programs, 16, 100000))



def channels_tests():
    parameters = [x for x in range(10, 2000, 1)]
    parameters2 = [100 for x in range(10, 2000, 1)]
    compile_tests(['cargo build --release --manifest-path=../Rust_channels/channels/Cargo.toml','cd ../Go_channels && go build && cd ../PlatformForBenchmarks'])
    programs = ["../Go_channels/channels","../Rust_channels/channels/target/release/channels"]
    
    names = ["Go","Rust"]
    
    results = run_programs_with_parameters(programs, parameters, parameters2)
    plot_results(results, parameters2, names, 'Liczba wątków', 'Czas trwania (s)','channel_tests')
    results.clear()


if __name__ == '__main__':
    channels_tests()
    sorting_tests3()
    mutex_tests2()
    