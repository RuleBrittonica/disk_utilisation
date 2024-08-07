

## Scripts

### `create_filesystem.py`

This script creates a dummy filesystem at a specified starting directory. You can control various parameters, including the maximum depth of the directory structure, the number of subfolders, the number of subfiles, and the maximum size of each file. After generating the filesystem, it produces a statistics file named `<start_folder>_stats.txt` containing:

- **Total Size of Filesystem:** The cumulative size of all files.
- **Total Number of Subfolders:** The total count of all subfolders.
- **Total Number of Subfiles:** The total count of all files.
- **Total Depth of Filesystem:** The maximum depth of the directory tree.
- **Number of Subfolders and Subfiles for Each Folder:** Details on the number of subfolders and subfiles for each folder.
- **List of All Files:** Paths of all files created.

**Usage:**

```bash
python create_filesystem.py <start_folder> <depth> <num_subfolders> <num_subfiles> <max_file_size>
```

### `cleanup_filesystem.py`

This script cleans up by deleting a specified directory and its contents, including any associated statistics file. It checks for the existence of the directory and the statistics file before attempting deletion.

**Usage:**

```bash
python cleanup_filesystem.py <directory>
```

### `python_install.sh`

This shell script ensures that Python and pip are installed on your system and then installs all required dependencies listed in `requirements.txt`. It performs the following checks:

1. Verifies if Python is installed.
2. Verifies if pip is installed.
3. Installs the dependencies using pip.

**Usage:**

Make sure to run chmod +x python_install.sh to set the script as executable before running it.

```bash
chmod +x python_install.sh  # Make the script executable
./python_install.sh
```