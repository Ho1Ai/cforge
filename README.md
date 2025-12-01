# CForge

CForge is a small application for C/C++ libraries

## User guide

Check if the library exists (on the server, not on your computer)  
```aiignore
cforge check-existence <lib_name>
```

Check short package info  
```aiignore
cforge fetch-data <lib_name>
```

Install a library  
```aiignore
cforge get <lib_name>
```

Remove a library  
```aiignore
# not ready yet
```

Output installed libraries information  
```aiignore
cforge libs-list
```

Output manual for library   
```aiignore
# not ready yet
```

Full user guide can be found on project website: [check full user guide]()

## Installation Guide

For Arch and Arch-based Linux distributions
```aiignore
yay -S cforge
```

For other distros:
```aiignore
git clone https://github.com/Ho1Ai/cforge
cd cforge/
chmod +x install.sh
./install.sh
```

## For developers
You can share your libraries using CForge. In order to upload a library, you have to use [this link]()