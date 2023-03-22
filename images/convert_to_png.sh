#!/bin/bash

# Convert all images in the current directory
# from PPM to PNG using ffmpeg
for i in *.ppm;
    do name=`echo $i | cut -d. -f1`;
    echo "$name"
    if [ -f "${name}.png" ]; then
        echo "PNG file exists, skipping conversion"
        continue
    fi
    ffmpeg -i "$i" "${name}.png";
done