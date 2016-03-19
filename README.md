# [Chat Server](https://kevinwilde.github.io/chat_server)

**What do we want to build?**

We want to build a chat server. Specifically, we want to create an application that will allow 
multiple users to access, send, and receive messages within one­on­one conversations through a 
server. This service would ideally also have a web interface and allow users to send file 
attachments.

**Why is this interesting?**

Implementing the chat server would require the use of multiple channels, mutex elements, and 
streams, all of which are structures that facilitate concurrency in a program. Not only would it be 
fulfilling for us to put our knowledge of these structures to the test, but to see that knowledge 
come together and create something that we’re familiar with will give us greater satisfaction than 
if we had developed some abstract new idea.

**What difficulties do we anticipate?**

While we now have some experience developing and working with a web server in Rust, none of 
us have any formal knowledge in the subject. As such, issues will likely arise with determining 
how to create a server to which multiple clients can connect.

The concurrent nature of the program, in that we must be able to send and receive all messages 
from multiple users at the same time, will probably be the trickiest part of the assignment. We 
must be able to properly assign roles and functions to each thread and determine how they will 
interact with each other to develop an effective chat system that handles multiple separate 
conversations.

Another difficulty that will arise is determining how to implement direct messaging in a way that 
allows a user to choose who they want to chat with when there are many other users logged on. 
We will need to find a way to show a user who else is logged on and available to chat, allow that 
user to select one of those people to chat with, notify that person that a user wants to chat with 
them, and then begin the communication between the two users.

**Concrete Requirements**

*Must-Have Features*
* Multiple clients must be able to connect to the server and chat.
* The server should be able to host multiple one­on­one conversations at a time.
* The clients and server must both be able to send and receive messages.
* Two clients who are chatting must see all messages between each other in the 
conversation.
* Messages must be displayed in chronological order.

*“Reach” Features*
* The chat service has a web interface.
* Clients can attach and download files from the chat.
* Some clients can have admin privileges and commands (e.g. ability to kick people from 
the server).
* Clients can initiate and participate in group (3+ people) messages.

**Use Cases and Examples**

*Example 1*

User A connects to the chat

User A is shown a list of other users who are available to chat

User A chooses to chat with User B

User B is notified that User A wants to chat

User A sends message “hello”

User B sends message “goodbye”

User A sends message “goodbye”

User B logs out

User A is shown a list of other users who are available to chat

User A logs out
