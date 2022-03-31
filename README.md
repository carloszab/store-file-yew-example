# Upload Blobs with Reqwest on Yew

Example to store blobs on Azure Storage using a Shared Access Signature, created from the file_upload [example](https://github.com/yewstack/yew/tree/master/examples/file_upload) from Yew, and tested on its version 0.19

## Install Rust

Run the following in your terminal, then follow the on-screen instructions to install [Rust](https://www.rust-lang.org/tools/install)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
## Install Trunk

```bash
cargo install --locked trunk
```

## Serve locally

```bash
trunk serve
```

## Usage
place your SAS url in the request, go to the served example in your browser on localhost:8080, toggle the read bytes flag and upload a png file, you should be able to check the response in the console or in the network tab of your browser


## Contributing
Pull requests are welcome.

## License
[MIT](https://choosealicense.com/licenses/mit/)