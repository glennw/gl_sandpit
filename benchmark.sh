#!/bin/bash
cargo build --release
./target/release/gl_sandpit null
./target/release/gl_sandpit clear1
./target/release/gl_sandpit clear2
./target/release/gl_sandpit clear4
./target/release/gl_sandpit clear8
./target/release/gl_sandpit quad1
./target/release/gl_sandpit quad2
./target/release/gl_sandpit quad4
./target/release/gl_sandpit quad8
./target/release/gl_sandpit clear1_quad1
./target/release/gl_sandpit clear1_quad2
./target/release/gl_sandpit clear1_quad3
./target/release/gl_sandpit clear1_quad4
./target/release/gl_sandpit clear1_quad5
./target/release/gl_sandpit clear1_quad6
./target/release/gl_sandpit clear1_quad7
./target/release/gl_sandpit clear1_quad8
./target/release/gl_sandpit clear1_quad9
./target/release/gl_sandpit clear1_quad10
