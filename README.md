# PRTGC_httpadvanced

An executable as replacement for PRTG's HttpAdvancedSensor.exe to use with Custom EXE Metascan template.

Background:

[The PRTG httpDataAdvanced sensor can not be used / created with a template utilizing template with custom EXE Metascan](https://kb.paessler.com/en/topic/68109-how-can-i-use-meta-scans-for-custom-exe-script-sensors)



In order to use a custom metascan to generate "HTTP Custom Advanced sensors", I had to make a "passthrough" executable.

It turned out to be fairly straight forward with Rust and reqwest.


USAGE:

prtgc_httpadvanced.exe [FLAGS] [OPTIONS] --method `<method>`

FLAGS:

-h, 	--help         Prints help information

-i,     --ignoressl    Ignore SSL error (Def false)

-L                 	   Use PRTG Linux Creds

-V,     --version      Prints version information

-W                 	   Use PRTG Windows Creds


OPTIONS:

-b, 	--body `<body>`          Post body

-m, 	--method `<method>`      HTTP Method (GET|POST|HEAD|OPTIONS) [default: GET]

-t, 	--timeout `<timeout>`    Query timeout (Def 30 sec)

        --url `<url>`            URL to get




I did some timing tests using my PRTG Gateway, and it turned out to be quite a bit faster...

All runs were executed against the same loopback "ping" using powershell "measure-command{....}"...

| Executabe              | ms | Ticks |Option|
| ---------------------- | -- | ----- |----- |
| HttpAdvancedSensor     |119 |1190968|      |
| prtgc_httpadvanced(ST) | 51 | 516736|reqwest/blocking|
| prtgc_httpadvanced(MT) | 43 | 434609|Tokio/MT, Workers 3|
| prtgc_httpadvanced(MT) | 40 | 402331|Tokio/flavor current_thread|




*** Please note, testing is not yet completed and will be u updated as it becomes available...

This application is provided as is, no warranty expressed or implied, use at your on peril....