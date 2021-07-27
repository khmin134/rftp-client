# rftp-client
The FTP client program written in Rust

## Usage
```
rftp-cliet
```

The default port is 21  
If you want to connect to another port, do as below

```
example.com:1234
127.0.0.1:1234
```

## Available commands
pwd   : Print working directory  
ls    : Print files located in the working directory  
cd    : Change working directory  
up    : Upload file to server  
down  : Download file from server  
mkdir : Make directory  
rm    : Remove a file  
rmd   : Remove a direcotry  
mv    : Change the name or path of a file or directory  
size  : Get the file size  

## License
MIT license (https://opensource.org/licenses/MIT)