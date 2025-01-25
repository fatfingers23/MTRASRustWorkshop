# Requirements
* rust
* Picotool binary for panic messages https://github.com/NotQuiteApex/picoboot-rs
* elf2uf2-rs for flashing and the fact that it can do the serial logging. Make sure to have example for bininstall since thats faster
* Vs code, rust anaylzer, even better toml, serial monitor, possibly use this video for part of it? https://www.youtube.com/watch?v=jq-zvM4Ih14

# Quick notes
1. The shared folder is shared code that isnt important besides helping with the process. do not put anything in it that needs to be gone over. exmaple if we need i2c devices in multiple bins just copy and paste. More important to keep it simple and abstract teh things away we are not teaching
2. Have 2 of each. Have like a startign template to work through, then the completed one. Maybe also have like a bonus for after the workshop?




# Lessons

## 01 Hello World

Every great adventure in programming starts with a Hello World! This is no difference.

Inside of [01_hello_world_begin.rs](/src/bin/01_hello_world_begin.rs) you will find the starting template.

todo should I write notes here or just use code comments? Should we type out things or rely on the bonus things?