import os
import shutil
import argparse

def cleanup_directory(path):
    # Delete the directory and its contents
    if os.path.exists(path):
        shutil.rmtree(path)
        print(f"Deleted directory: {path}")
    else:
        print(f"Directory {path} does not exist.")

    # Delete the stats file if it exists
    stats_file = path + "_stats.txt"
    if os.path.exists(stats_file):
        os.remove(stats_file)
        print(f"Deleted stats file: {stats_file}")
    else:
        print(f"Stats file {stats_file} does not exist.")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Cleanup a directory and its stats file.")
    parser.add_argument('directory', type=str, help="The directory to clean up.")
    args = parser.parse_args()

    cleanup_directory(args.directory)
