# Resize my Svg

Display your svg image and resize it with a slider.

<img src="./preview.png" width="250" height="250" />

## Limits

Please notice that as for now there's a little bug : the image must not set width and height inside the svg code.

## Building

### AppImage in a Docker container (Linux)

To build the AppImage in a Docker container (you may want to launch as sudo-user/root) :

1. `docker build -t resize_my_svg .`
2. `docker run --rm -v $(pwd)/output:/output -it resize_my_svg /bin/bash`

Inside the terminal of your container run :
1. `/home/docker_user/copy_executable.sh`
2. `exit`

## Credits

Using crab image from https://www.svgrepo.com/svg/396221/crab.

Using open image from https://www.svgrepo.com/svg/501815/open-open-a-file.