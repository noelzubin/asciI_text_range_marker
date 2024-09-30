# ASCII Text Range Marker
This project marks ranges of text in a given string based on specified start and end positions.

``` sh
echo "root:*:0:0:System Administrator:/var/root:/bin/sh
0:3:Username
5:5:Password
7:7:User Id(UID)
9:9:Group Id(UID)
11:30:User Info
32:40:Home Directory
42:48:Command/shell" | cargo run 
```

Output:
```
 ╭ Username                                                 
 │   ╭ Password                                             
 │   │ ╭ User Id(UID)                                       
 │   │ │ ╭ Group Id(UID)            ╭ Home Directory        
 │   │ │ │          ╭ User Info     │        ╭ Command/shell
╭┴─╮ │ │ │ ╭────────┴─────────╮ ╭───┴───╮ ╭──┴──╮
root:*:0:0:System Administrator:/var/root:/bin/sh
```

