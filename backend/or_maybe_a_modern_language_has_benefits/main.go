package main

import (
    "bufio"
    "fmt"
    "log"
    "net"
    "strings"
    "os"
    "strconv"
)

func main() {
    port, err := strconv.Atoi(os.Args[1])
    ln, err := net.Listen("tcp", fmt.Sprintf(":%d", port))
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("Listening on port %d\n", port)
    conn, err := ln.Accept()
    if err != nil {
        log.Fatal(err)
    }
    for {
        message, err :=  bufio.NewReader(conn).ReadString('\000')
        if err != nil {
            log.Fatal(err)
        }
        fmt.Print("Message Received:", string(message))
        newmessage := strings.ToUpper(message)
        conn.Write([]byte(newmessage + "\n"))
    }
}
