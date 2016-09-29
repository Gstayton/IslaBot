Tentative structure layout, non-binding document

Server Struct
    -Hostname: &str
    -Port: &str
    -Nick: &str
    -Channels: <Vec>
    -cmdChar: Char

Message Struct
    -Sender: &str
    -Server: &str
    -Contents: <Command Struct>
    -Time: <Date Object> // Assuming there is a native one
    -Raw: &str // Completely unedited message from the server

Message Implementation
    -new(&str) -> Message Struct


Command Struct
    -Command: &str
    -Parameters: <Vec<&str>>
        -Last parameter will always being the 'trailing' if it existed (It should)
        -Order for arguments is retained in <Vec>
    

