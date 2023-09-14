# Md4

## Md4 Architecture
> Md4 backend is implemented in **rust** and frontend is implemented in **go**.
>
> Backend is responsible for:
> - Taking input from frontend
> - Generating md4 hash
> - Sending hash to frontend send output file
>
> Frontend is responsible for:
> - Taking input from user
> - Sending input to backend using file
> - Receiving output from backend
> - Comparing output file with expected output file
> - Printing success or failure message
> - Printing Other Statistics like input size, output size, digest values, etc..,

## Limitations
1. Tested only on linux
2. Input file size should be less than 1GB (This is because of the limitation of the frontend. Frontend uses
temporary file to send input to backend. This temporary file is created in /tmp directory. So, if the input file size is greater than 1GB, then the temporary file will be created in /tmp directory and it will be deleted after the execution of the program.Thus it can consume all the space in /tmp directory. So, I recommend the input file size to 1GB. This can be fixed by using some other method to send input to backend. But, I don't have time to do that.)