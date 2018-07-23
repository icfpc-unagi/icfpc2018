# ICFPC2018
Team Unagi's repository for ICFPC 2018


## Members

- Takuya Akiba
- Kentaro Imajo
- Hiroaki Iwami
- Yoichi Iwata
- Toshiki Kataoka
- Naohiro Takahashi


## Build and Run

### Requirements
* Rust
* C++
* Bazel
* Mono

### Build
```
cargo build --release
make install
```

### Rust

Example for assembly problems:
```
./target/release/solve2 <TARGET_MODEL_FILE> a 40 x | ./bin/trace_binarize <OUTPUT_TRACE_FILE>
```
Examples for disassembly problems:
```
./target/release/destroy_iwiwi '' <SOURCE_MODEL_FILE> > <OUTPUT_TRACE_FILE>
```
```
./target/release/destroy_tos '' <SOURCE_MODEL_FILE> > <OUTPUT_TRACE_FILE>
```

### C#

Example:
```
mono ./bin/chokudai-solver/014.exe <TARGET_MODE_FILE> | ./target/release/run_postproc <TARGET_MODE_FILE> /dev/stdin | ./bin/trace_binarize <OUTPUT_TRACE_FILE>
```

### Shell

Examples (for reassembly problems)
```
./bin/solvers/iwiwi-004+chokudai-014 <TARGET_MODEL_FILE> <SOURCE_MODEL_FILE> > <OUTPUT_TRACE_FILE>
```
```
./bin/solvers/iwiwi-004+wata-008 <TARGET_MODEL_FILE> <SOURCE_MODEL_FILE> 40 x > <OUTPUT_TRACE_FILE>
```


## Solution Approcach

### Solutions for Assembly Problems

#### Solution 1

`wata/src/bin/solve2.rs`

Our construction solver uses the following strategy. We construct one or two layers at a time (e.g., construct the layer y=0, 1, 2, ...). In order to keep everything grounded, we simultaneously construct support pillars. These supports are deconstructed at the end of the construction. For constructing a layer, we first devide the voxels in the layer into several regions. Then each robot greedily chooses a near region and fills the region by using GFill operations.

#### Solution 2

`chokudai/Program.cs`

Basically we create each layer, one by one, from the bottom one.
To keep the groundedness, it postpones some parts that requires to go down.
With this technique, it can output solutions without harmonics for all assembly inputs, although it is not guaranteed that it can work with any inputs.
When it is able to create large rectangles,
it greedily enumerate rectangles, and uses GFill operations.

### Solutions for Disassembly Problems

#### Solution 1

`wata/src/bin/destroy_iwiwi.rs`

We place bots in a 2D grid form, and repeat plane gvoid (with offset (0, -1, 0)).
By using four steps, they destruct the plane directly below them.
Then, they go down, and remove the next plane.
If the field is large, it repeats the above procedure at different places.

#### Solution 2

`wata/src/bin/destroy_tos.rs`

This disassembles relatively small models by 3-dim GVoid operations. Firstly the model is projected onto xz-plane and then divided into 31x31 cells. 
If there's no 2x2 block of cells, there are free lines for nanobots to work.

Example: Full voxels in the model `problemsF/FD083_src.mdl` only exist in the following 3 cells out of 5x5.
```
..... -> z [0, 1, 17, 48, 69, 70]
..#..
..#..
..#..
.....

|
v
x [0, 1, 7, 38, 69, 70]
```

### Solutions for Reassembly Problems

For each reassembly problem, we applied disassembly and assembly solvers for the source and target models, respectively.
Then, we just concatenated the outputs of them.

### Other

#### fusion_all
With this routine, all bots fusion into a bot and move to the origin, at last parts of algorithms.  Basically each bot follows BFS-search from the origin.  To reduce traffic jams, further 1-step search is done.

#### fission_to
By reversing the fusion_all routine, an algorithm can start with any configuration of initial (reachable) positions.
