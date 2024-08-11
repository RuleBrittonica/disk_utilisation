const { invoke } = window.__TAURI__.tauri;

document.addEventListener('DOMContentLoaded', async () => {
  try {
    // Fetch the list of drives from the Rust backend
    const drives = await invoke('get_drives');

    const leftColumn = document.querySelector('.left-column');

    // Create "All Drives" button
    const allDrivesButton = document.createElement('button');
    allDrivesButton.className = 'drive-button';
    allDrivesButton.textContent = 'All Drives';
    leftColumn.appendChild(allDrivesButton);

    // Function to format bytes to the appropriate unit
    function formatBytes(bytes) {
      if (bytes >= 1e12) {
        return (bytes / 1e12).toFixed(2) + ' TB';
      } else if (bytes >= 1e9) {
        return (bytes / 1e9).toFixed(2) + ' GB';
      } else if (bytes >= 1e6) {
        return (bytes / 1e6).toFixed(2) + ' MB';
      } else if (bytes >= 1e3) {
        return (bytes / 1e3).toFixed(2) + ' KB';
      } else {
        return bytes + ' B';
      }
    }

    // Create buttons dynamically based on the drives
    drives.forEach(drive => {
      const button = document.createElement('button');
      button.className = 'drive-button';

      // Create a container for drive info and progress bar
      const driveInfoContainer = document.createElement('div');
      driveInfoContainer.className = 'drive-info-container';

      // Add icon
      const icon = document.createElement('img');
      icon.src = drive.icon; // Use the icon path from the backend
      icon.className = 'icon';
      driveInfoContainer.appendChild(icon);

      // Add drive name
      const text = document.createElement('span');
      text.textContent = drive.name;
      driveInfoContainer.appendChild(text);

      // Add progress bar and space details
      const progressBarContainer = document.createElement('div');
      progressBarContainer.className = 'progress-bar-container';

      const progressBar = document.createElement('div');
      progressBar.className = 'progress-bar';

      const usedSpace = drive.total_space - drive.available_space;
      const usagePercentage = (usedSpace / drive.total_space) * 100;

      const progressBarInner = document.createElement('div');
      progressBarInner.className = 'progress-bar-inner';
      progressBarInner.style.width = `${usagePercentage}%`;

      progressBar.appendChild(progressBarInner);
      progressBarContainer.appendChild(progressBar);

      // Display space details
      const spaceDetails = document.createElement('span');
      spaceDetails.className = 'space-details';
      spaceDetails.textContent = `${formatBytes(usedSpace)} / ${formatBytes(drive.total_space)}`;

      progressBarContainer.appendChild(spaceDetails);
      driveInfoContainer.appendChild(progressBarContainer);

      button.appendChild(driveInfoContainer);

      button.addEventListener('click', () => {
        alert(`You clicked ${drive.name}`);
        // Handle drive selection
      });

      leftColumn.appendChild(button);
    });
  } catch (error) {
    console.error('Error fetching drives:', error);
  }

  // Add a new section for memory usage
  const bottomLeft = document.createElement('div');
  bottomLeft.className = 'bottom-left';
  document.body.appendChild(bottomLeft);

  const progressBarContainer = document.createElement('div');
  progressBarContainer.className = 'memory-progress-bar-container';

  const progressBar = document.createElement('div');
  progressBar.className = 'memory-progress-bar';

  const progressBarInner = document.createElement('div');
  progressBarInner.className = 'memory-progress-bar-inner';
  progressBar.appendChild(progressBarInner);

  const memoryDetails = document.createElement('span');
  memoryDetails.className = 'memory-details';

  bottomLeft.appendChild(progressBarContainer);
  progressBarContainer.appendChild(progressBar);
  bottomLeft.appendChild(memoryDetails);

  // Function to update memory usage
  async function updateMemoryUsage() {
    try {
      const memoryUsage = await invoke('get_memory_usage');
      const usedMemoryPercentage = (memoryUsage.used_memory / memoryUsage.max_memory) * 100;
      progressBarInner.style.width = `${usedMemoryPercentage}%`;
      memoryDetails.textContent = `Memory Used: ${memoryUsage.used_memory} MB / ${memoryUsage.max_memory} MB`;
    } catch (error) {
      console.error('Error fetching memory usage:', error);
    }
  }

  // Update memory usage every second
  setInterval(updateMemoryUsage, 1000);

  // Initial update
  updateMemoryUsage();
});
