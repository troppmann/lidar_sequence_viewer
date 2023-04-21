# LiDAR Sequence Viewer
Renders the lidar data from the KITTI Benchmark[1,2].


#### Features
- Renders lidar sequence data smoothly 
- Sequence can be paused/played/advanced/fastened/slowed like a youtube video 
- Low memory consumption
- Fully controllable camera
- Labels can be changed anytime
- Single executable, without installer

https://user-images.githubusercontent.com/74185941/233684697-85746a16-c2c5-4345-9683-36c1af6f9866.mp4

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
- structure
- small example_sequence 140MB
- link to release
- whole data set 80GB http://www.semantic-kitti.org/dataset.html#download
- license of example sequence Creative Commons Attribution-NonCommercial-ShareAlike 



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
Lidar Sequence Viewer is dual-licensed under either:
- [MIT License](../main/LICENSE-MIT)
- [Apache License, Version 2.0](../main/LICENSE-APACHE) 

#### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

#### References
[1]&nbsp;&nbsp;&nbsp;Andreas Geiger, Philip Lenz, and Raquel Urtasun. Are we ready for Autonomous Driving? The KITTI Vision Benchmark Suite. In Proceedings of the IEEE Conference on Computer Vision and Pattern Recognition (CVPR), 2012

[2]&nbsp;&nbsp;&nbsp;Andreas Geiger, Philip Lenz, Christoph Stiller, and Raquel Urtasun. Vision meets Robotics: The KITTI Dataset. International Journal of Robotics Research (IJRR), 2013


https://github.com/PRBonn/semantic-kitti-api
