# my-decoder

This app is a decoder based in a secret key to find a message.
It's develop with RUST

## command to build docker image 

`docker build -t my-decoder -f .\DockerFile .`

## run image

The image need an input from multiple lines, so you can send the input from a file, with the next command.
Where text.txt is a file in the local computer (you can see an example in the project), so you need a file in your local to test it

`cat test.txt | docker run --rm -i my-decoder`
