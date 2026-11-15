#! /bin/bash
hyperfine --metrics time_wall_clock:s -M 3 './target/release/implementation'
