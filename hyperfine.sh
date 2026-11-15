#! /bin/bash
hyperfine --metrics time_wall_clock:s,memory_peak_resident:kB -M 3 './target/release/implementation'
