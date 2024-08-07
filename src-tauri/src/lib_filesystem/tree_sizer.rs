use crate::lib_filesystem::filesystem::*;


/*
The goal:
- Starting with either a Computer, Disk or Folder, traverse down the whole tree.
- From the bottom up, populate out the file sizes for any File object, one
    folder at a time. This is done one row at a time from the bottom of the tree to the top
- After each recursion up the tree, return the new Folder object
    - This is so that we can send this updated object through to the frontend and have it
        appropriately display the object and the user gets to see the filesystem populate
        out in real time

*/