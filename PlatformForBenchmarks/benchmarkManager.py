import subprocess
import time
import matplotlib.pyplot as plt
from tqdm import tqdm
from datetime import datetime


def compile_tests(programs):
    for program in programs:
        subprocess.call(program, shell=True)

def measure_execution_time(command):
    start_time = time.time()
    subprocess.call(command, shell=True)
    end_time = time.time()
    execution_time = end_time - start_time
    return execution_time

def run_programs_with_parameters(programs, parameters):
    results = {}
    for program in programs:
        results[program] = []
        for param in tqdm(parameters):
            command = f"{program} {param}"  # Tworzenie polecenia uruchamiającego program z parametrem
            execution_time = measure_execution_time(command)
            results[program].append(execution_time)
    return results

def plot_results(results, parameters, program_names):
    i = 0
    for program, times in results.items():
        plt.plot(parameters, times, label=program_names[i])
        i+=1
    plt.xlabel('Liczba wątków/gorutyn')
    plt.ylabel('Czas trwania (s)')
    plt.legend()

    # Zapisywanie wykresu do pliku o nazwie zgodnej z dzisiejszą datą i godziną
    current_datetime = datetime.now().strftime("%Y-%m-%d_%H-%M-%S")
    filename = f"wykres_{current_datetime}.png"
    plt.savefig(filename)

if __name__ == '__main__':
    
    #programs = ['go run ../Go/test.go', 'cargo run --quiet --manifest-path ../Rust/test_project/Cargo.toml']  # Nazwy programów
    #programs = ['go run ../Go/matrixv2.go', 'cargo run --quiet --manifest-path ../Rust/matrixv2/Cargo.toml']  # Nazwy programów
    #programs = ['cargo run --quiet --manifest-path ../Rust/matrixv2/Cargo.toml']

    parameters = [x for x in range(1, 2000)]  # Przykładowe parametry
    #compile_tests(['cargo build --release --manifest-path=../Rust/matrixv2/Cargo.toml','cd ../Go && go build && cd ../PlatformForBenchmarks'])
    compile_tests(['cargo build --release --manifest-path=../Rust_benchmark/matrix/Cargo.toml','cd ../Go_benchmark && go build && cd ../PlatformForBenchmarks'])
    programs = ["../Go_benchmark/matrix 1","../Rust_benchmark/matrix/target/release/matrix 1"]
    names = ["Go","Rust"]
    results = run_programs_with_parameters(programs, parameters)
    plot_results(results, parameters, names)
