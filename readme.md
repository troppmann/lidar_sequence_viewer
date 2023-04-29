# LiDAR Sequence Viewer
Renders the lidar data from the KITTI Benchmark[1,2].


#### Features
- Renders lidar sequence smoothly 
- Sequence can be paused/played/advanced/fastened/slowed like a YouTube video 
- Low memory consumption
- Fully controllable camera
- Labels can be changed anytime
- Single executable, without installer

https://user-images.githubusercontent.com/74185941/233689260-8876c7af-5834-45e7-b621-24de6f3a5e1c.mp4

#### Controls
###### Camera
Press and hold the `Right Mouse Button` to control the cameras view.
To move the camera use:
| Key           | Function | | Key | Function |
|:-----:   | ----------- | - | :-: | -----|
| `w`        | Forward | | `q`     | Up
| `s`     | Backwards |  | `e`     | Down |
| `a`     | Left |  | | |
| `d`     | Right |  | | |

Press and hold `Shift` to speedup movement. 

###### Sequence Player
| Key           | Function |
|:-----:   | ----------- |  
|`Space`   | Play \| Pause | 
| `→`     | Next Frame |
| `←`     | Previous Frame |

#### Data
A sequence is structured as follows:
```
sequence
├── velodyne
│   ├── 000000.bin
│   ├── 000001.bin
│   ├── 000002.bin
│   └── ...
└── labels
    ├── 000000.label
    ├── 000001.label
    ├── 000002.label
    └── ...
```
Each frame of a sequence is stored in a separate file. The frame number is the filename. 
A valid sequence starts with the filename 000000 and increases the number by 1 for each frame. The data of a frame is further divided:
- '######.bin' file containing the points position and remission
- '######.label' file is optional and contains  the classification and object id
  
To get the whole SemanticKitty dataset(~80GB) with 22 sequences follow the instructions on [www.semantic-kitti.org](http://www.semantic-kitti.org/dataset.html#download), its also a great source for additional information.

A small example sequence(140MB) can be downloaded from the [release page](TODO). The example sequence contains 100 frames extracted from sequence 08 of the SemanticKitty dataset and follows the [Creative Commons Attribution-NonCommercial-ShareAlike](https://creativecommons.org/licenses/by-nc-sa/4.0/) license.

#### Build from source
A prebuilt executable can be found on the Release page, or the project can easily be build.
To compile the project `rust` and `cargo` have to be installed. To setup `rust` and `cargo` follow the instructions on [www.rust-lang.org](https://www.rust-lang.org/tools/install).

```bash
# inside project folder
$ cargo run --release
# the executable is located under ./target/release
```
#### Todo
- [ ] Allow remapping of keys
- [ ] Short tutorial at first startup

#### License
LiDAR Sequence Viewer is dual-licensed under either:
- [MIT License](../main/LICENSE-MIT)
- [Apache License, Version 2.0](../main/LICENSE-APACHE) 

#### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

#### References
[1]&nbsp;&nbsp;&nbsp;J. Behley and M. Garbade and A. Milioto and J. Quenzel and S. Behnke and C. Stachniss and J. Gall. SemanticKITTI: A Dataset for Semantic Scene Understanding of LiDAR Sequences. Proc. of the IEEE/CVF International Conference on Computer Vision (ICCV), 2019

[2]&nbsp;&nbsp;&nbsp;Andreas Geiger, Philip Lenz, and Raquel Urtasun. Are we ready for Autonomous Driving? The KITTI Vision Benchmark Suite. In Proceedings of the IEEE Conference on Computer Vision and Pattern Recognition (CVPR), 2012

TODO https://github.com/PRBonn/semantic-kitti-api
