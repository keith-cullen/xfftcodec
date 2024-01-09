xfftcodec
=========

An encoder/decoder using the xfft package.

Instructions
------------

1. Download an input wave file

        $ curl https://www.mediacollege.com/audio/tone/files/100Hz_44100Hz_16bit_30sec.wav -o in.wav

2. Run the unit tests

        $ cargo test

3. Run the codec

        $ cargo run -- --in <infile> --out <outfile> --cpus <cpulist> --num <number-of-new-samples-per-window>
