# making_waves

Cute little library for making music out of synth waves. Currently you can specify a sequence of pulses defined by a pitch, volume, and duration. These are encoded into raw floats and can be played using `ffplay`

## Todo
- Add encoding of song with tempo for easy construction of songs using beats instead of durations for each note.
- Add support for encoding multitrack songs. 
- Experiment with percussive sounds
- Add support for different wave forms (currently just sin wave)
- Add support for reading a song specification from a file
- Investigate better ways of encoding sound data  