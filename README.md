# 1brc
Parse 1 billion rows as fast as possible

## ideas
- ~~BufReader increase capacity~~ marginal speed improved, doesn't do anything
    - ~~rational: less system calls, more stored in memory, less time spend doing sys calls~~
- ~~Implement my own parse for floats, built in parse has a lot of overhead~~ shavedsome time w this!!
- Faster hashmap, figure out how rust hashmap works and implement smth better
- ~~Don't use structs, avoid struct overhead, use tuple instead?~~ Not true, structs same speed, if not faster
- BTreeMap instead of sort?
    - look into how it works
- ~~Replacement for lines?~~
    - ~~sus that lines read reads until \n and returns after, could just read directly till \n~~ replaced with read_until ';' and until '\n' after

