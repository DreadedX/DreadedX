# Hi! I'm Tim Huizinga
An Applied Physics student with a passion for programming!

I have always enjoyed programming as a hobby, and would love to make it my career.
Since then I have also combining this hobby with the hardware side of things.
Most recently I have picked up [Rust], and have fallen in love with this programming language.

I'm also quite experienced with Linux as I have been daily driving it for the past decade at this point.
Making me very familiar with the terminal and the different command line tools available.
And have even been running my own Linux server at home for quite some time now!

[Rust]: https://rust-lang.org


## Projects
### [Z80 Computer](https://git.huizinga.dev/Z80/Z80)
The first big hardware project that I worked on was building a computer, from the ground up, around the [Z80] microprocessor.
I had to learn a wide range of skills for this project, including things like learning how to design PCBs, programming in assembly, working with [FPGA]s, and learning to operate an oscilloscope.

[Z80]: https://en.wikipedia.org/wiki/Zilog_Z80
[FPGA]: https://en.wikipeida.org/wiki/Field-programmable_gate_array

### [Car Stereo](https://git.huizinga.dev/Dreaded_X/car-stereo)
My Peugeot 207 only has bluetooth for calling, so I decided it would be fun to build my own bluetooth receiver using the [ESP32] microcontroller.
The original goal was to just build an audio receiver and hook it up to the aux port in my glovebox.
However since then I have also connected the [ESP32] to the [CAN bus] of my car, allowing me to use the controls on my steering wheel to control the music.

[ESP32]: https://en.wikipedia.org/wiki/ESP32
[CAN bus]: https://en.wikipedia.org/wiki/CAN_bus

### [Home Automation](https://git.huizinga.dev/Dreaded_X/automation_rs)
I have slowly been converting my house into my very own smart home!
It all started with a couple of [Philips Hue] light bulbs.
The Hue app allows for some level of automation but it wasn't quite doing what I wanted.
So initially I wrote a very simple program in [Go] to add my own automations.
As I added more smart devices to my house, the program grew massively in scope with things quickly getting hacked in just to make it work.
Eventually I decided to rewrite the whole thing in [Rust]!
This was my first real Rust after picking up the language during [Advent of Code] and was (and still is) a great learning experience.

[Philips Hue]: https://nl.wikipedia.org/wiki/Philips_Hue
[Go]: https://go.dev
[Rust]: https://rust-lang.org
[Advent of Code]: https://adventofcode.com/

### [Pico P1](https://git.huizinga.dev/Dreaded_X/pico_p1)
This is my most recent project, as I had recently decided to pick up a [Raspberry Pi Pico W] just to play around with.
I decided to build a device to read out my [DSMR5] based smart meter using it's P1 port and publish the information using [MQTT].
The main intention of this project is to learn about Rust for embedded devices, which is still a very new ecosystem.
So far it has been quite a nice experience!

[Raspberry Pi Pico W]: https://en.wikipedia.org/wiki/Raspberry_Pi#Raspberry_Pi_Pico
[DSMR5]: https://www.netbeheernederland.nl/_upload/Files/Slimme_meter_15_a727fce1f1.pdf
[MQTT]: https://nl.wikipedia.org/wiki/MQTT

