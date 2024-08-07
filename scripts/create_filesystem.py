import os
import random
import string
import argparse

def create_dummy_filesystem(start_path, current_depth, num_subfolders, num_subfiles, max_file_size, stats, max_depth):
    if current_depth < 1:
        return

    os.makedirs(start_path, exist_ok=True)

    num_folders = 0
    num_files = 0
    folder_stats = {}

    subfolder_list = []
    for _ in range(random.randint(num_subfolders // 2, num_subfolders)):
        subfolder_name = "folder_" + ''.join(random.choices(string.ascii_letters, k=5))
        subfolder_path = os.path.join(start_path, subfolder_name)
        subfolder_list.append(subfolder_path)
        # Recursive call to create subfolders
        create_dummy_filesystem(subfolder_path, current_depth - 1, num_subfolders, num_subfiles, max_file_size, stats, max_depth)
        num_folders += 1

    # Randomize the number of files created in this folder
    num_files = random.randint(num_subfiles // 2, num_subfiles)
    for _ in range(num_files):
        file_name = "file_" + ''.join(random.choices(string.ascii_letters, k=5)) + ".txt"
        file_path = os.path.join(start_path, file_name)
        file_size = random.randint(1, max_file_size)
        with open(file_path, 'w') as file:
            file_content = ''.join(random.choices(string.ascii_letters + string.digits, k=file_size))
            file.write(file_content)
        stats['all_files'].append(file_path)
        stats['total_size'] += file_size

    folder_stats['path'] = start_path
    folder_stats['depth'] = current_depth
    folder_stats['num_subfolders'] = num_folders
    folder_stats['num_subfiles'] = num_files
    stats['folder_stats'].append(folder_stats)
    stats['total_folders'] += num_folders
    stats['total_files'] += num_files

    # Update the maximum depth
    if current_depth > max_depth[0]:
        max_depth[0] = current_depth

def write_stats_to_file(stats, start_folder):
    output_file = f"{start_folder}_stats.txt"
    with open(output_file, 'w') as file:
        file.write(f"Total Size of Filesystem: {stats['total_size']} bytes\n")
        file.write(f"Total Number of Subfolders: {stats['total_folders']}\n")
        file.write(f"Total Number of Subfiles: {stats['total_files']}\n")
        file.write(f"Total Depth of Filesystem: {stats['total_depth']}\n")
        file.write("\nNumber of Subfolders and Subfiles for Each Folder:\n")
        for folder in stats['folder_stats']:
            file.write(f"Folder: {folder['path']}\n")
            file.write(f"  Depth: {folder['depth']}\n")
            file.write(f"  Number of Subfolders: {folder['num_subfolders']}\n")
            file.write(f"  Number of Subfiles: {folder['num_subfiles']}\n")
        file.write("\nList of All Files:\n")
        for file_path in stats['all_files']:
            file.write(f"{file_path}\n")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Create a dummy filesystem.")
    parser.add_argument('start_folder', type=str, help="The start folder path.")
    parser.add_argument('depth', type=int, help="The maximum depth of subfolders.")
    parser.add_argument('num_subfolders', type=int, help="The number of subfolders.")
    parser.add_argument('num_subfiles', type=int, help="The number of subfiles.")
    parser.add_argument('max_file_size', type=int, help="The maximum size of subfiles.")
    args = parser.parse_args()

    stats = {
        'total_size': 0,
        'total_folders': 0,
        'total_files': 0,
        'total_depth': 0,  # Initialize total depth
        'folder_stats': [],
        'all_files': []
    }

    # Use a list to track the maximum depth (mutable object to pass by reference)
    max_depth = [0]

    create_dummy_filesystem(args.start_folder, args.depth, args.num_subfolders, args.num_subfiles, args.max_file_size, stats, max_depth)
    stats['total_depth'] = max_depth[0]  # Set the total depth in stats
    write_stats_to_file(stats, args.start_folder)