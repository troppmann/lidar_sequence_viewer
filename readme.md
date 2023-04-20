# LiDAR Sequence Viewer
Renders the lidar data from the KITTI Benchmark[1,2].


#### Features
- Renders lidar sequence data smoothly 
- Sequence can be paused/played/advanced/fastered/slowered like a youtube video 
- Low memory consumption
- Fully controllable camera
- Labels can be changed anytime

https://user-images.githubusercontent.com/74185941/`232229283-82436fdd-b555-477a-b34b-e764feb4a4ba.mp4




https://user-images.githubusercontent.com/74185941/232229340-910e3a21-e183-4ecc-8f24-753d7a5a3f34.mp4

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
######Sequence Player
| Key           | Function |
|:-----:   | ----------- |  
|`Space`   | Pause \| Play | 
| `→`     | Next Frame |
| `←`     | Previous Frame |

#### Data
- structure
- small example_seqeunce 140MB
- link to release
- whole data set 80GB http://www.semantic-kitti.org/dataset.html#download
- license of example sequence Creative Commons Attribution-NonCommercial-ShareAlike 



#### Build from source
- rust & cargo installed by following https://www.rust-lang.org/tools/install 
- executeable location
```bash
# inside project folder
$ cargo run --release
```
#### Todo
- [ ] Allow remmaping of keys
- [ ] Short tutorial at first startup

#### License
- dual licenced under apache and MIT
- contribution

#### References
[1]&nbsp;&nbsp;&nbsp;Andreas Geiger, Philip Lenz, and Raquel Urtasun. Are we ready for Autonomous Driving? The KITTI Vision Benchmark Suite. In Proceedings of the IEEE Conference on Computer Vision and Pattern Recognition (CVPR), 2012

[2]&nbsp;&nbsp;&nbsp;Andreas Geiger, Philip Lenz, Christoph Stiller, and Raquel Urtasun. Vision meets Robotics: The KITTI Dataset. International Journal of Robotics Research (IJRR), 2013


https://github.com/PRBonn/semantic-kitti-api
