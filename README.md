# OdinsEye

OdinsEye is a Program to help beginners learn to use the UNIX Shell as easily as possible.

The application itself is written in Rust, because it is pretty low-level and therefore ideal for reading and writing from and to the terminal.

## How it works

As soon as the program is executed it will generate a new folder in the users home-directory. This directory will act as the **/**-directory in typical UNIX systems.

The application will send a GET-requets to our backend and retrieve a JSON-file containing all information necessary to build an exercise for the users to do.

Such a file typically contains the exercises **id, name, root, and folder/file-structure.**

## Installation

No installation needed! Just visit [Our Website](http://caretaker.wurzer.cc:9040/#/info) and download the binary file.

## Usage

To run the application just execute the binary file from any terminal by typing **./odins_eye**

### Startup

When the application is first launched you will be greeted by a startup screen which will provide you with useful information on how to use the program.

### Selecting an exercise

By typing the command **odin exercise** you can select an exercise to do. 

### Access documentation

On selection of an exercise you will recieve instructions on how to finish it.

