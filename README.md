# dbfc
Distrubted batch file conversion

## Usage

Initialize dbfc in a root directory for a hierarchy of files to convert.

```
$ dbfc job encode_videos_job
Created new job encode_videos_job
Current job is now encode_videos_job
```

Add a conversion rule for a file type:

```
# dbfc rule -t [FILE_EXTENSION] -c [COMMAND]

$ dbfc rule -t mp4 -c 'ffmpeg -i $file_path -c:v libx265 $out_file_path.mp4'
```

Run conversion:

```
$ dbfc run
    Running encode_videos_job...
        20080915.avi
        20081019.avi
    3 of 7 converted, 0%
        press [q] to quit
```