# sshnip

After switching to an Android phone, I was missing the cross device copy paste magic 🪄 that my iPhone had.
Works a treat for MFA codes, so I mostly needed it one way, i.e. phone to laptop.
Whenever I copied an MFA code or something else, I opened the app manually, yes, this is a bodge. 

Full disclaimer. After what seemed like a bricked phone, an incorrect diagnosis of a Samsung service center and a following unnecessary repair, switched back to iPhone.

## Client

Simple Android app which starts up the first time to show a form to enter the API URL and API key.  
The next time it starts up it simply grabs the content from the clipboard and if it's text, sends it over to the server and then closes. 
Initially I wanted to use a `Service`, but Android apps can only grab clipboard content if they are in the foreground, so `Activity` it was `¯\_(ツ)_/¯`

## Server

Simple Rocket server with a `GET` and `POST` endpoint for fetching and storing clipboard contents. Contents are stored in memory and cleared after `GET`.

## Shortcut

A Shortcut that would run a shell script to `curl` the clipboard contents and pipe to `pbcopy` storing the remote clipboard into the laptops clipboard.

<img width="618" height="270" alt="image" src="https://github.com/user-attachments/assets/4c3c476a-9e6d-4f44-9410-867fbbdbc747" />

_The fact that this is even possible on something made by Apple is so funny to me. Kudos to the engineers who built Shortcuts._
