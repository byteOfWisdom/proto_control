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

func err_check(err error) {
    if err != nil { log.Fatal(err) }
}

func main() {
    port, err := strconv.Atoi(os.Args[1])
    err_check(err)

    ln, err := net.Listen("tcp", fmt.Sprintf(":%d", port))
    err_check(err)

    fmt.Printf("Listening on port %d\n", port)
    conn, err := ln.Accept()
    err_check(err)

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
