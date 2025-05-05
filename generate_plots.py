import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import os

def plot_csv_statistics(directory_path):
    # Ensure the 'plots' subdirectory exists
    plots_dir = os.path.join(directory_path, 'plots')
    os.makedirs(plots_dir, exist_ok=True)

    # Identify CSV files in the directory
    csv_files = [f for f in os.listdir(directory_path) if f.endswith('.csv')]
    if not csv_files:
        raise FileNotFoundError("No CSV file found in the specified directory.")
    csv_path = os.path.join(directory_path, csv_files[0])

    # Read the CSV file
    data = pd.read_csv(csv_path, header=None, names=['Epoch', 'Max_Weight', 'Cycles_Spent'])

    # Plot: Epoch vs. Max Weight
    plt.figure(figsize=(10, 6))
    plt.plot(data['Epoch'], data['Max_Weight'], linestyle='-', linewidth=1.5, color='blue')
    plt.title('Epoch vs. Max Weight')
    plt.xlabel('Epoch')
    plt.ylabel('Max Weight')
    plt.grid(True)
    plt.savefig(os.path.join(plots_dir, 'epoch_vs_max_weight.png'))
    plt.close()

    # Plot: Epoch vs. Cycles Spent with Trend Line
    plt.figure(figsize=(10, 6))
    plt.plot(data['Epoch'], data['Cycles_Spent'], 
            linestyle='-', linewidth=1.5, color='red', 
            label='Actual Cycles')
    
    # Calculate trend line
    x = data['Epoch']
    y = data['Cycles_Spent']
    z = np.polyfit(x, y, 1)
    p = np.poly1d(z)
    
    plt.plot(x, p(x), linestyle='--', linewidth=2, color='blue',
            label=f'Trend ({"Increasing" if z[0] > 0 else "Decreasing"})')
    
    plt.annotate(f'Slope: {z[0]:.2f}', 
                xy=(0.05, 0.95), 
                xycoords='axes fraction',
                fontsize=10,
                bbox=dict(boxstyle="round", fc="white"))
    
    plt.title('Epoch vs. Cycles Spent with Trend Analysis')
    plt.xlabel('Epoch')
    plt.ylabel('Cycles Spent')
    plt.grid(True)
    plt.legend()
    plt.savefig(os.path.join(plots_dir, 'epoch_vs_cycles_spent.png'))
    plt.close()

    print(f"Plots saved in: {plots_dir}")

def process_csv_and_generate_plots(directory):
    """
    Recursively traverses the given directory to find CSV files and generates plots for each.
    """
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith('.csv'):
                csv_path = os.path.join(root, file)
                plots_dir = os.path.join(root, 'plots')
                os.makedirs(plots_dir, exist_ok=True)

                # Read the CSV file
                data = pd.read_csv(csv_path, header=None, names=['Epoch', 'Max_Weight', 'Cycles_Spent'])

                # Plot: Epoch vs. Max Weight
                plt.figure(figsize=(10, 6))
                plt.plot(data['Epoch'], data['Max_Weight'], linestyle='-', linewidth=1.5, color='blue')
                plt.title('Epoch vs. Max Weight')
                plt.xlabel('Epoch')
                plt.ylabel('Max Weight')
                plt.grid(True)
                plt.savefig(os.path.join(plots_dir, 'epoch_vs_max_weight.png'))
                plt.close()

                # Plot: Epoch vs. Cycles Spent with Trend Line
                plt.figure(figsize=(10, 6))
                plt.plot(data['Epoch'], data['Cycles_Spent'], 
                        linestyle='-', linewidth=1.5, color='red', 
                        label='Actual Cycles')
                
                # Calculate trend line
                x = data['Epoch']
                y = data['Cycles_Spent']
                z = np.polyfit(x, y, 1)
                p = np.poly1d(z)
                
                plt.plot(x, p(x), linestyle='--', linewidth=2, color='blue',
                        label=f'Trend ({"Increasing" if z[0] > 0 else "Decreasing"})')
                
                plt.annotate(f'Slope: {z[0]:.2f}', 
                            xy=(0.05, 0.95), 
                            xycoords='axes fraction',
                            fontsize=10,
                            bbox=dict(boxstyle="round", fc="white"))
                
                plt.title('Epoch vs. Cycles Spent with Trend Analysis')
                plt.xlabel('Epoch')
                plt.ylabel('Cycles Spent')
                plt.grid(True)
                plt.legend()
                plt.savefig(os.path.join(plots_dir, 'epoch_vs_cycles_spent.png'))
                plt.close()

                print(f"Plots generated for: {csv_path}")

def rename_stg_to_csv(directory):
    """
    Recursively renames all .stg files to .csv within the given directory and its subdirectories.
    """
    for root, _, files in os.walk(directory):
        for filename in files:
            if filename.endswith('.stg'):
                old_path = os.path.join(root, filename)
                new_filename = filename[:-4] + '.csv'
                new_path = os.path.join(root, new_filename)
                os.rename(old_path, new_path)
                print(f'Renamed: {old_path} -> {new_path}')

if __name__ == "__main__":
    #  source venv/bin/activate             
    # python3 generate_plots.py                                                                           

    # Exemplo de uso:
    # rename_stg_to_csv('/home/matheus/STG/results/')
    # plot_csv_statistics('/home/matheus/STG/results/500')
    # process_csv_and_generate_plots('/home/matheus/STG/results/protostg/100')
    process_csv_and_generate_plots('/home/matheus/STG/results/1500')