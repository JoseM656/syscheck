# Welcome


syscheck is a lightweight binary rust for linux than give you varius tool, like monitor tools and other utilities like copy paths and files. 
The project uses clap to join the varius tools.

![screenshot](screenshots/screenshot_1.png)


## Tools


The number of tools increases over time; they are mainly divided into 
monitoring tools and various utilities.

The standar input of the program is:

```
syscheck <command> --FLAG
```


### cpu


This tool just prints the percentage of use of the processor, admits "--all" flag and "--ghz" for only frequency.

```
[syscheck - CPU - all]:
- Usage: 4.03%
- Freq: 0.52 GHz
```


### mem


This tool displays the main system memories: RAM, swap, and cache. It supports the "--all", "--cache", 
and "--swap" flags.

```
[syscheck - mem - all]:
- RAM used: 4476 MB (4 GB)
- Total RAM: 7616 MB (7 GB)
- SWAP used: 228 MB (0 GB)
- Cached: 2924 MB (2 GB)
```



## Advantages

- Just one lightweight binary.
- Memory safe thanks to Rust.
- It can be integrated into any ecosystem.
- It doesn't use any libraries other than clap and fs for reading "/proc".
- Scalable, there is no limit to the number of tools.
