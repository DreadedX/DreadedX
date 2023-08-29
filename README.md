# Hi! I'm Tim Huizinga
An Applied Physics student with a passion for programming!

I have always enjoyed programming as a hobby, and would love to make it my carreer.
Since then I have also combining this hobby with the hardware side of things.
Most recently I have picked up [Rust](https://rust-lang.org), and have fallen in love with this programming language.

I'm also quite experienced with Linux as I have been daily driving it for the past decade at this point.
Making me very familiar with the terminal and the different command line tools available.
And have even been running my own Linux server for quite some time now!

## Projects
### [Z80 Computer](https://git.huizinga.dev/Z80/Z80)
The first big hardware project that I work on was building a computer, from the ground up, around the [Z80](https://en.wikipedia.org/wiki/Zilog_Z80) microprocessor.
I had to learn a wide range of skills for this project, including things like learning how to design PCBs, programming in assembly, working with FPGAs, and learning to operate an oscilloscope.

### [Car Stereo](https://git.huizinga.dev/Dreaded_X/car-stereo)
My Peugeot 207 only has bluetooth for calling, so I decided it would be fun to build my own bluetooth receiver using the [ESP32](https://en.wikipedia.org/wiki/ESP32) microcontroller.
The original goal was to just build an audio receiver and hook it up to the aux port in my glovebox.
However since then I have also connected the ESP32 to the [CAN bus](https://en.wikipedia.org/wiki/CAN_bus) of my car, allowing me to use the controls on my steering wheel to control the music.

### [Home Automation](https://git.huizinga.dev/Dreaded_X/automation_rs)
I have slowly been converting my house into my very own smart home!
It all started with a couple of Philips Hue light bulb, the Hue app allows for some level of automation but it wasn't quite doing what I wanted.
So initially I wrote a very simple program in [Go](https://go.dev).
As I added more smart devices to my house, the program grew massively in scope with things quickly getting hacked in just to make it work.
Eventually I decided to rewrite the whole thing in Rust!
This was my first real Rust after picking up the language during [Advent of Code](https://adventofcode.com/) and was (and still is) a great learning experience.

### [Pico P1](https://git.huizinga.dev/Dreaded_X/pico_p1)
This is my most recent project, as I had recently decided to pick up a [Raspberry Pi Pico W](https://en.wikipedia.org/wiki/Raspberry_Pi#Raspberry_Pi_Pico) just to play around with.
I decided to build a P1 reader to read out my [DSMR5](https://www.netbeheernederland.nl/_upload/Files/Slimme_meter_15_a727fce1f1.pdf) based smart meter.
The main intention of this project is to learn about Rust for embedded devices, which is still a very new ecosystem.
So far it has been quite a nice experience!
