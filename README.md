# dbfc
Distrubted batch file conversion

## Usage

Initialize a dbfc batch job for the current directory:

```
$ dbfc init batch_job1 -d=/path/to/output/files
```

Edit `.dbfc/batch_job1.bj` by adding a rule to the `rules` list. In this example, the rule will run for all files of type `avi`. The `ffmpeg` program will be ran as a child process provided it is installed and in the PATH. The variable `$file_path` will be replaced at runtime with the path to the file being processed. The variable `$file_path_out` will be replaced with a new filepath using the same filename but with a path inside the initialized destination directory.

```
"rules": {
    "avi": "ffmpeg -i $file_path -c:v libx264 -preset slow $file_path_out"
}
```

Run the batch job:

```
$ dbfc run batch_job1
```

## Rule Variables

### file_path

The absolute path to the file being processed.

### file_path_out

The absolute path to where the file should be written to.
