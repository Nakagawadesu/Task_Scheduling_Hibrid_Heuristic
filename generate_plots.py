import pandas as pd
import matplotlib.pyplot as plt
import os
def plot_csv_statistics(directory_path):
      # Ensure the 'plots' subdirectory exists
    plots_dir = os.path.join(directory_path, 'plots')
    os.makedirs(plots_dir, exist_ok=True)

    # Identify the CSV file in the provided directory
    csv_files = [f for f in os.listdir(directory_path) ]
    if not csv_files:
        raise FileNotFoundError("No CSV file found in the specified directory.")
    csv_path = os.path.join(directory_path, csv_files[0])

    # Read the CSV file
    data = pd.read_csv(csv_path, header=None, names=['Epoch', 'Max_Weight', 'Cycles_Spent'])

    # Plot: Epoch vs. Max Weight
    plt.figure()
    plt.plot(data['Epoch'], data['Max_Weight'], marker='o')
    plt.title('Epoch vs. Max Weight')
    plt.xlabel('Epoch')
    plt.ylabel('Max Weight')
    plt.grid(True)
    plt.savefig(os.path.join(plots_dir, 'epoch_vs_max_weight.png'))
    plt.close()

    # Plot: Epoch vs. Cycles Spent
    plt.figure()
    plt.plot(data['Epoch'], data['Cycles_Spent'], marker='o', color='r')
    plt.title('Epoch vs. Cycles Spent')
    plt.xlabel('Epoch')
    plt.ylabel('Cycles Spent')
    plt.grid(True)
    plt.savefig(os.path.join(plots_dir, 'epoch_vs_cycles_spent.png'))
    plt.close()

    print(f"Plots saved in: {plots_dir}")

def process_csv_and_generate_plots(directory):
    """
    Recursively traverses the given directory to find CSV files and generates plots for each.

    :param directory: The root directory to start the search.
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
                plt.figure()
                plt.plot(data['Epoch'], data['Max_Weight'], marker='o')
                plt.title('Epoch vs. Max Weight')
                plt.xlabel('Epoch')
                plt.ylabel('Max Weight')
                plt.grid(True)
                plt.savefig(os.path.join(plots_dir, 'epoch_vs_max_weight.png'))
                plt.close()

                # Plot: Epoch vs. Cycles Spent
                plt.figure()
                plt.plot(data['Epoch'], data['Cycles_Spent'], marker='o', color='r')
                plt.title('Epoch vs. Cycles Spent')
                plt.xlabel('Epoch')
                plt.ylabel('Cycles Spent')
                plt.grid(True)
                plt.savefig(os.path.join(plots_dir, 'epoch_vs_cycles_spent.png'))
                plt.close()

                print(f"Plots generated for: {csv_path}")
def rename_stg_to_csv(directory):
    """
    Recursively renames all .stg files to .csv within the given directory and its subdirectories.

    :param directory: The root directory to start the renaming process.
    """
    for root, _, files in os.walk(directory):
        for filename in files:
            if filename.endswith('.stg'):
                old_path = os.path.join(root, filename)
                new_filename = filename[:-4] + '.csv'  # Replace .stg with .csv
                new_path = os.path.join(root, new_filename)
                os.rename(old_path, new_path)
                print(f'Renamed: {old_path} -> {new_path}')

# RUN first   source venv/bin/activate  
# rename_stg_to_csv('/home/matheus/STG/results/')        
#         
# plot_csv_statistics('/home/matheus/STG/results/500')
# 
process_csv_and_generate_plots('/home/matheus/STG/results/500')