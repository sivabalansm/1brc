# 1brc
Parse 1 billion rows as fast as possible

## Run time from benchmarks
- 70s on my m5 mac air

## ideas
- BufReader increase capacity
    - rational: less system calls, more stored in memory, less time spend doing sys calls
- Implement my own parse for floats, built in parse has a lot of overhead
- Faster hashmap, figure out how rust hashmap works and implement smth better
