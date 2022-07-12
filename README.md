# my-decoder

This app is a decoder based in a secret key to find a message.
It's develop with RUST

## command to build docker image 

`docker build -t my-decoder -f .\DockerFile .`

## run image

The image need an input from multiple lines, so you can send the input from a file, with the next command.
Where text.txt is a file in the local computer (you can see an example in the project), so you need a file in your local to test it

`cat test.txt | docker run --rm -i my-decoder`

or you can run with the image docker:

`cat test.txt | docker run --rm -i karlitasg7/my-decoder`

## run test
You can execute the test with the next command

`cargo test`