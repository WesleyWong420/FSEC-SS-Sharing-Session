# FSEC-SS Sharing Session

## Rust for Reverse Engineering
### Setup
Installing Ghidra
```
sudo apt install ghidra
```

### Download Files
Download Rust Binary - [flagchecker](https://cloudmails-my.sharepoint.com/:u:/g/personal/tp059618_mail_apu_edu_my/EWqiSpmUthFBmtikj21JL6gBPBhOW2VIMDmpGv8f0HH-sw?e=7MRhgY)

### Walkthrough
1. Untick `Create Address Table` under Analysis Options (Analysis will take around ~10 minutes to complete)
2. Locate Main Function (Different entry point than classic C programs)
3. Locate user-defined functions under `Symbol Tree` > `Namespaces`
4. Import Ghidra Scripts under `Windows` > `Script Manager` (Step 5 & 6)
5. Demangle Rust symbols using [DemangleRust.py](https://gist.githubusercontent.com/str4d/e541f4c28e2bca80d222434ac1a204f4/raw/80688c7458284310b7cad445ce94333a0ae969ef/DemangleRust.py)
6. Analyze Crate dependencies using [RustDependencyStrings.py](https://raw.githubusercontent.com/BinaryDefense/GhidraRustDependenciesExtractor/main/RustDependencyStrings.py)
