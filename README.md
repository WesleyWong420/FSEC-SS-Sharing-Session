# FSEC-SS Sharing Session
![](./img/logo.png)

## Rust for Reverse Engineering
### Setup
Installing Ghidra
```
sudo apt install ghidra
```
### Download Files
Download Rust Binary - [flagchecker](https://cloudmails-my.sharepoint.com/:u:/g/personal/tp059618_mail_apu_edu_my/EWqiSpmUthFBmtikj21JL6gBPBhOW2VIMDmpGv8f0HH-sw?e=7MRhgY) (63.4MB)
```
┌──(kali💀JesusCries)-[~/Desktop/flagchecker/target/debug]
└─$ sha256sum flagchecker 
f6a7bea70e44d0ae8f7990f327b7be537e28cf81f3c5df400677c3f8294ed9ec  flagchecker
```

### Walkthrough
1. Untick `Create Address Table` under Analysis Options (Analysis will take around ~10 minutes to complete)
2. Locate Main Function (Different entry point compared to classic C programs)
3. Locate user-defined functions under `Symbol Tree` > `Namespaces`
4. Import Ghidra Scripts under `Windows` > `Script Manager` (Step 5 & 6)
5. Demangle Rust symbols using [DemangleRust.py](https://gist.githubusercontent.com/str4d/e541f4c28e2bca80d222434ac1a204f4/raw/80688c7458284310b7cad445ce94333a0ae969ef/DemangleRust.py)
6. Analyze Crate dependencies using [RustDependencyStrings.py](https://raw.githubusercontent.com/BinaryDefense/GhidraRustDependenciesExtractor/main/RustDependencyStrings.py)

### Tips
1. Search for interesting strings
2. Analyze `.rodata` section under `Program Trees`
3. Analyze the usage of `reqwest` and `rot13` in combination with other user-defined functions found under `Namespaces`
4. Refer to Rust Package Registry for [reqwest](https://crates.io/crates/reqwest) and [rot13](https://crates.io/crates/rot13)

### General RE Tips for Rust
* We can benefit from the fact that [Rust is an open source language](https://github.com/rust-lang/rust/)
* there is a `Cargo` repository with lots of Rust packages and `.rodata` section contains some links to them. For example, the string 
```"/root/.cargo/registry/src/github.com-{id}/reqwest"``` gives us a clue about [a library of higher level HTTP client](https://crates.io/crates/reqwest)
* There are links not only to Rust packages, but also to particular files in those packages.

## Cryptography - RSA Variations
### Setup 
Installing Sage
```
git clone https://github.com/sagemath/sage.git
cd sage
make configure
./configure
make
./sage
```

Installing Python Libraries
```
pip3 install pycryptodome
```

Troubleshooting
[Sagemath Installation in DebianOS](https://pikaroot.github.io/blogs/2023-02-06-Sagemath_Installation_without_git_clone_repo)

## Boot2Root
### Setup
```
git clone https://github.com/WesleyWong420/FSEC-SS-Sharing-Session.git
cd FSEC-SS-Sharing-Session/Docker
chmod +x build.sh
./build.sh
```

### Initial Access & Test Connection
```
docker exec -it knockknock /bin/bash

root@47efb31454af:/# ifconfig

nc <docker0-IP> 9002
```

### Download Files
Download PCAP File - [tcpdump.cap](./tcpdump.cap) (1.2KB)

### Walkthrough
> You will receive the flag by connecting to port 9002 if all the conditions are met.
1. Enumerate installed services: `service --status-all`
2. Replay knock sequence as observed from `tcpdump.cap`: `knock <docker0-IP> 7000:tcp 8000:udp 9000:tcp`
3. Loot flag!
