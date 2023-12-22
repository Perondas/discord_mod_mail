# Discord Workshop Update Tracker Bot

This is a simple bot that allows messages to be sent to a specific channel

## Commands
* /register_channel - Register a channel to send update messages to
* /message - Send a message to the registered channel
* /anonymous_message - Send an anonymous message to the registered channel

All the commands are purely slash commands.
They can also all be configured to be used by a specific role only.

## Features: 
* Total permission control over all commands

## Permissions
The bot requires the following permissions:
* Send Messages

## Setup
Rename the .evn.example file to .env and fill in the values. The bot will not work without this file.
Run docker compose up to start the bot.
