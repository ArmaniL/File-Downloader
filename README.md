A simple file donwloader in Rust. It either takes a singular url or a text file containing a list of urls. It downloads the file(s) and saves it to the current directory. if providing a list of files you must make sure that the list is delimited by a new line

**EXAMPLE**
Downloader www.google.com
Downloader /desktop/listofwebsites.txt

*TODO*
Add progress bar for download
Download files concurently
